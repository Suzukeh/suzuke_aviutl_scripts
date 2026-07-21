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

---

## 3. 波線グラデーション_Su 高速化

### W-1 【効果:大】全色空間共通: 端点色の事前変換を CPU 側で実施

psmain 内の `color_space` switch（cases 1-7）では、`col1`/`col2`（sRGB）を選択された色空間に変換してから `lerp()` している。この変換（`srgb2linear`, `srgb2hsv`/`srgb2hsl`, `linear2lab`, `linear2lch`, `linear2oklab`, `linear2oklch`）は**全ピクセルで同一**の uniform 値に対する計算である。

- Lua 側で `color_space` に応じた Lua 版変換関数を用意し、`color1`/`color2` を事前変換したベクトルを uniform として渡す
- シェーダー内では `lerp(pre_converted_col1, pre_converted_col2, t)` のみになり、`pow()`/行列乗算/`atan2` 等の uniform 再計算が全ピクセルで消滅
- 特に cases 4-7（Lab/Oklab 系）で効果大: 行列 2 回・`pow` 4 回・`atan2` 1 回相当の演算を削減
- 逆変換（`lab2linear`→`linear2srgb` 等の lerp 結果の変換）は t に依存するためシェーダー内に残す

### W-2 【効果:中高】2 パス描画の 1 パス統合

現在は **psmain（グラデーションレンダリング）→ mask（元画像アルファでクリッピング）** の 2 回のフルスクリーンピクセルシェーダーが走っている。

- `psmain` に 2 枚目のテクスチャ（`cache:wavegrad_orig_*`）を `register(t1)` で追加バインドし、シェーダー内で元画像の alpha を読んで出力に乗算する
- mask シェーダー（`mask`）とそれを呼ぶ Lua 側パス（line 374-376）が不要になる
- フルスクリーンレンダーパス 1 回分の帯域 + ドローコール削減

### W-3 【効果:大・静止時】パラメータ不変時のグラデーションキャッシュ

psmain の出力は**完全に手続き的**（uniform + `pos.xy` のみに依存。`src` テクスチャは宣言されているがシェーダー内で未参照）。

- 全パラメータ（color1/2, color_space, LEN, AMP, PHA, Type, Gap, gradient_w, Method, CenterX/Y, Rotate, w, h）のハッシュ/連結文字列をキャッシュキーとする
- キー一致時は psmain 呼出しをスキップし、前フレームの tempbuffer 結果を再利用
- マスクパス（line 376）は元画像が変化しうるため毎フレーム実行（ただし W-2 統合後は psmain 内で完結）
- 用途想定: パラメータ無アニメの静止グラデーションオーバーレイ — psmain の全コスト消滅

### W-4 【効果:小】sincos の cos 未使用を sin に置換

`wave_range()` の case 0（正弦波）と case 4（正弦波絶対値）で `sincos(dir, s, c)` を呼んでいるが `c` は未使用。

- `return sin(dir) * AMP` / `return abs(sin(dir)) * AMP` に置換
- 無駄なレジスタ消費とコサイン計算を削減

### W-5 【効果:小】psmain が src テクスチャを読んでいない

- psmain は `Texture2D src : register(t0)` を宣言しているが本文で `Sample`/`Load` していない
- Lua 側の line 348-349（`drawtarget tempbuffer` + `draw()`）は無駄
  - ただし W-2 統合時に 2 枚目のテクスチャとして活用できる

---

## 4. @文字種別制御_Su 高速化

### TC-BUG 【重要:バグ修正】@範囲指定 で `parse_ranges` が前方参照で nil

line 537 で `parse_ranges(rng)` を呼んでいるが、`function parse_ranges(s)` は line 555 で定義されている。Lua の前方参照ルールにより line 524-539 の `for i=1,4` ループ実行時には `parse_ranges` が nil → **常にクラッシュ**。

- `parse_ranges` と `in_ranges` 関数を `grp_cfg` 構築ループより上に移動するだけ

### TC-1 【効果:大】テキスト処理結果のフレーム間キャッシュ

3 エフェクト全てで、毎フレーム UTF-8 分割 → 文字種分類 → バッチグループ化 → 制御文字列生成 を実行している。静止テキスト（最も一般的な用途）では結果が同一。

- `_G.__CharType_<effect_id>` に `{key, styled, base_fn, base_sz}` を保存
- キャッシュキー: 生テキスト + 全スタイルパラメータ（fontsel, fontsize, gw, gh, align, tfacesel, tc, oc, 各カテゴリの en/fn/cl/sz/rot/sc/ox/oy）
- ヒット時: 文字パース（全 3 工程）を全てスキップし、`obj.setfont` + `obj.load("text", cached_styled)` のみ実行
- 効果: 文字数 100 以上で顕著。500 文字のテキストなら毎フレーム 500 回の `get_ctype` + バッチ比較ループをカット

### TC-2 【効果:中高】`type_lookup` テーブルによるバッチループの O(7) → O(1) 化

内側バッチループ（lines 184-203 / 358-379 / 622-644）で、隣接文字のスタイルを毎回 `cfg.en` 判定 + `math.floor(base_sz*cfg.sz/100)` + `string.format` 等 7 つの式で再計算している。

- `type_cfg`/`grp_cfg` 構築後に、全文字種×全グループ分の解決済みスタイルを `type_lookup[tp]` テーブルに事前計算
- バッチループ内の隣接文字比較は単なるテーブル参照比較に（計算ゼロ）
- 文字数 100 の場合、約 600 回の不要な演算が消滅

### TC-3 【効果:中】文字種判定結果の文字単位メモ化

`get_ctype` / `get_grp` / `utf8_decode` の各関数は純粋（同じ文字に同じ結果）。文字数 × 2 回呼ばれる（先頭 + 隣接）。

- `local cache = {}` で `{ [char] = type }` のメモ化テーブルを用意
- 同じ文字が繰り返し出現する日本語テキストで特に効果的
- TC-1（フレームキャッシュ）が効く場合は不要だが、キャッシュミス時の保険

### TC-4 【効果:中】text_reader モジュールのロードキャッシュ

3 エフェクト全てで毎フレーム `pcall(function() return obj.module("text_reader") end)` を実行。

- `.mod2` のロードは初回のみ。モジュールレベル変数にキャッシュ
- `local _text_reader = false`; 初回のみ `pcall(obj.module, "text_reader")` → nil or handle を保存
- 毎フレームの DLL ルックアップ + pcall オーバーヘッドを削減

### TC-5 【効果:小】その他

- 改行正規化 `gsub("\\n","\n"):gsub("\r\n","\n"):gsub("\r","\n")` → `gsub("\\n","\n"):gsub("\r\n?","\n")` で 3→2 パス
- 3 エフェクトでコピペされているテキスト取得ブロックを共通関数に抽出（保守性向上、キャッシュ化が容易に）
- `@範囲指定` の `get_grp` で `in_ranges` が線形探索 — レンジ数が少ないため問題にならないが、TC-3 のメモ化でカバー

---

## 優先度（全スクリプト統合版）

1. **TC-BUG**（文字種別制御）— クラッシュバグ、最優先
2. **TC-1**（文字種別制御）— 静止テキストで 95%+ 削減、最も効果が大きい
3. **Z-1 + Z-2**（Z変位）— 静止画用途で劇的、低リスク
4. **W-1**（波線グラデーション）— 全色空間で per-pixel の無駄な uniform 変換を除去
5. **C-3 + C-1 検証**（CacheMask）— ぼかし動的ループ + GPU 完結転送
6. **W-2**（波線グラデーション）— 2 パス→1 パス統合
7. **TC-2 + TC-3 + TC-4**（文字種別制御）— TC-1 導入後の保険的改善
8. **W-3**（波線グラデーション）— 静止グラデーションで有効（TC-1 同様のパターン）
9. **Z-3**（Z変位 Phase 2.5）— 高分割数の実用化が必要になってから
10. **W-4, W-5** — 微細な改善。W-1/W-2 実装時に同時対応

## 作業ルール

- style.md のルールに従い、高速化の工夫はコード内コメントで明記する（可読性を無視してよい代わりに、どのような工夫をしたのか説明する）
- 実機検証前に `obj.getinfo("script_time")` でブロック別の baseline を取得し、高速化後に比較する
