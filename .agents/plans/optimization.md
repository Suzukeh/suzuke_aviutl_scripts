# 高速化計画

> 策定: 2026-07-20

## 背景

最近実装したスクリプト（Z変位_Su.anm2 v0.1.0, @CacheMask_Su.anm2 v1.4.0）の高速化余地を調査。
両スクリプトとも未検証のため、高速化は実機検証と同時進行し、`obj.getinfo("script_time")` でブロック別の前後比較を取る。

## 調査で確定した制約（公式型定義 `lua_aviutl_definitions/aviutl2.lua` より）

1. **`obj.getpixeldata` は `lightuserdata`（ポインタ）を返す** → Lua でのバイト解析は**不可能**（`ffi` も利用禁止）。一括解析には `.mod2` (Rust) が必須と確定。
2. `obj.putpixeldata` も同様にポインタを受け取る。両者とも公式に **「VRAM 転送のため速くない」と明記**。
3. `pixeloption("get")` の対象は `"object"`/`"framebuffer"` のみ → **`tempbuffer` を直接 `getpixel` できない**。
4. `cache:` バッファはフレームを跨げない（知識ベース既知）。**同一フレーム内でオブジェクト間共有できるかは未検証**（既存実装は全て getpixeldata 経由）。

---

## 1. Z変位_Su 高速化

### Z-1 【効果:大】ジオメトリのフレーム間キャッシュ

典型的用途（静止画 + 回転/カメラだけ動かす）では、毎フレーム生成される `polys`/`walls` は完全に同一。
それを毎回「縮小マップ描画 → ぼかし → getpixel 二重ループ → ポリゴン配列構築」しているのが最大の無駄。

- `--check@cache_geo:ジオメトリキャッシュ,false` を追加（静止画向け。動く画像では OFF 必須）
  - 必要な旨を `--information` や説明で明記
- `_G.__ZHen_i_<effect_id>` に `{ key, polys, walls }` を保存
  - key = `strength, div_x, div_y, face_mode, base, blur, culling, skip_transparent, w, h` の連結文字列
- ON かつキー一致時は以下を**全てスキップ**:
  - obj 変数（ox, oy, oz, cx, cy, cz, zoom, sx, sy, rx, ry, rz, aspect）の退避・復元
  - `copybuffer("cache:orig", "object")`（フルサイズコピー）
  - `drawpoly` による縮小マップ描画（tempbuffer）
  - `obj.effect("ぼかし")`
  - `pixeloption("get", "object")`
  - getpixel 二重ループ（最大 3.7 万回/フレーム）
  - `copybuffer("object", "cache:orig")`（フルサイズ復元）
  - polys/walls テーブル構築
  - → `drawpoly` のみ実行
- キャッシュミス時（初回 or パラメータ変更）は通常の全処理

### Z-2 【効果:中】純 Lua ループ改善（Z-1 OFF 時の高速化でもある）

- **Z グリッド＋透明フラグの事前計算**: 連続面モードで `to_z()` が頂点あたり最大 4 回呼ばれている → 1 回に。側面モードの隣接セル Z 再計算もグリッド参照に置換
- **座標・UV の事前計算**: セル毎の除算/乗算（192² で約 30 万回/フレーム）を列配列 `xs[i]`/`us[i]`（nx+1 要素）と行配列 `ys[j]`/`vs[j]`（ny+1 要素）に引き上げ
- `lum`/`alp` 2 テーブルを z グリッドに一本化し、ループ内の `has_alpha` 分岐をループ外へ
- `local getpixel = obj.getpixel`、`local abs = math.abs` 等の関数ローカル化（@DataEffect_Su の既存パターンに倣う）

### Z-3 【効果:大・高分割数向け】`.mod2` による一括サンプリング（Phase 2.5 の具体化）

- 現状は `getpixel` を最大 (192+1)² ≈ **3.7 万回/フレーム** API 呼出 → `getpixeldata` 1 回 + Rust 側解析に置換
- Rust 側で RGBA 解析 → 輝度 → Z 配列 `Vec<f64>` を返却。欲を言えば polys/walls 配列そのものを構築して Lua のポリゴン構築ループも削減
- `pcall(require)` でモジュール不在時は純 Lua 処理にフォールバック
- ※ `getpixeldata` は lightuserdata 返却のため純 Lua 解析は不可と確定

### 実装後のファイル更新

- `.agents/progress/Z変位_Su.md`（状況・対応内容・TODO 更新）
- `.agents/plans/Z変位_Su.md`（Phase 1.5 として反映済み、完了時に状態補完）
- `.agents/progress.md`（一覧更新）

---

## 2. @CacheMask_Su 高速化

### C-1 【効果:大・要実機検証】Capture→Apply 転送の GPU 内完結化

現状は毎フレーム **VRAM→CPU 読出し（Capture の `getpixeldata`）→ CPU→VRAM 書込み（Apply の `putpixeldata`）** という往復が発生。公式が「速くない」と明記する経路。

- `--select@transfer:転送モード=1,GPUキャッシュ=1,互換(getpixeldata)=0` を @Capture/@Apply 双方に追加
- **GPU モード**:
  - @Capture: `obj.copybuffer("cache:mask_"..id, "object")` に置換
  - サイズ受け渡し: 現在書き込みのみの死にテーブル `_CacheMask_Su[mask_id] = {w, h}`（@Capture 15行目）を活用
  - @Apply: `cache:mask_N` を ps_apply のリソースに直接バインド。getpixeldata/putpixeldata/tempbuffer 経由を全て省略
  - blur パスはそのまま（既に純 GPU 処理）
- **互換モード**: 現行のまま（getpixeldata → _G → putpixeldata）
- **検証項目**:
  - `cache:` が同一フレーム内でオブジェクト間共有できるか
  - マスクオブジェクトが下位レイヤー（先にレンダリング）の場合の描画順
  - Capture 未実行フレームでの Apply のフォールバック挙動
  - → 検証結果に基づき既定モードを決定

### C-2 【効果:中】静的マスクの再キャプチャスキップ（互換モード側の保険）

- @Capture に `--check@per_frame:毎フレーム更新,true`（IDgen_Su と同パターン）
- false かつ `_G` にデータ済みなら getpixeldata をスキップ → 読出しコスト消滅
- ただし Apply の putpixeldata（書込み側）は `cache:` がフレーム跨げないため残る
  - C-1 が不可だった場合の保険として価値あり

### C-3 【効果:中・ぼかし使用時】ps_blur の動的ループ境界

- 現状: HLSL の blur 内で `for (int i = -64; i <= 64; i++)` の固定 129 タップ × 2 パス（重み 0 でも `Sample` 実行）
- `for (int i = -r; i <= r; i++)` の動的境界に変更（r は cbuffer の `radius` 由来）。例: ぼかし 4 なら 9 タップ（約 1/14）
- HLSL ps_4_0 以降で動的ループ可。実機検証時に動作確認（古い GPU で動かない場合の備え）

### 実装後のファイル更新

- `.agents/progress/CacheMask_Su.md`（状況・対応内容・TODO 更新）
- `.agents/progress.md`（一覧更新）

---

## 優先度

1. **Z-1 + Z-2**（Z変位 v0.2.0）— 静止画用途で劇的、低リスク
2. **C-3 + C-1 検証**（CacheMask v1.5.0）— C-1 が効けば毎フレームの VRAM 往復が消滅
3. **Z-3**（Phase 2.5）— 高分割数の実用化が必要になってから着手
4. **C-2** — C-1 が不可だった場合の保険

## 作業ルール

- style.md のルールに従い、高速化の工夫はコード内コメントで明記する（可読性を無視してよい代わりに、どのような工夫をしたのか説明する）
- 実機検証前に `obj.getinfo("script_time")` でブロック別の baseline を取得し、高速化後に比較する
