# 文字種別制御_Su — 引き継ぎドキュメント

## 概要

テキストの文字種（漢字/ひらがな/カタカナ/英字/数字/その他、全角/半角、Unicode範囲指定）ごとに
フォント・サイズ・色・回転・変形・位置オフセットをAviUtl2の制御文字で設定するアニメーション効果。

## ファイル構成

```
Suzuke/
├── @文字種別制御_Su.anm2          # メインスクリプト（3セクション: @文字種別, @全角半角, @範囲指定）
├── text_reader.mod2                # Rust製mod2プラグイン（テキストオブジェクトから文字列読取）
├── text_reader_mod2/               # mod2のRustソース
│   ├── Cargo.toml
│   └── src/lib.rs
├── .agents/knowledge/
│   ├── aviutl2-text-control-chars.md       # AviUtl2テキスト制御文字リファレンス
│   ├── aviutl2-scripting-principle.md      # obj.draw()禁止原則
│   └── aviutl2-plugin.md                   # mod2作成ガイド
└── README.md                       # （本ドキュメント）
```

## アーキテクチャ

### 3セクション構成

| セクション | 分類方式 | グループ |
|-----------|---------|---------|
| `@文字種別` | 漢字/ひらがな/カタカナ/英字/数字/その他 | 6グループ |
| `@全角半角` | Unicodeコードポイントで全角/半角判定 | 2グループ |
| `@範囲指定` | ユーザー指定のUnicode範囲(Hex) | 4グループ |

### 描画方式

`obj.load("text", styled_string)` の**1回呼び出し**で描画。AviUtl2の制御文字を文字列に埋め込む方式。

```
<gw字間><gh行間>  <@フォント><sサイズ><#色><tr回転><tw変形><th変形><p+ox,+oy> 文字列  <p><tw><th><tr><#><s><@>  \n  ...
```

- **tempbuffer不使用**
- **obj.draw()不使用**（後続エフェクトを阻害しない）
- **obj.setfont() → obj.load("text", styled)** の1回で描画完了

### 各種別の制御文字マッピング

| 制御文字 | パラメータ名 | 型 |
|---------|------------|-----|
| `<#RRGGBB>` | `X_cl` | `--color@` |
| `<sサイズ>` | `X_sz` | `--track@` (基準サイズに対する%) |
| `<@フォント>` | `X_font` | `--select@` (-1=基準に従う) |
| `<tr角度>` | `X_rot` | `--track@` (度) |
| `<tw><th>` | `X_sc` | `--track@` (%) |
| `<p+ox,+oy>` | `X_ox` / `X_oy` | `--track@` (px) |
| `<gw>` / `<gh>` | `gw` / `gh` | `--track@` (全体) |

### mod2プラグイン (text_reader.mod2)

Rust製。`aviutl2` crate使用。

```rust
// 旧API（互換性維持）
fn get_text(layer: usize, frame: usize, section: &ReadSection) -> String

// 新API（フォント・サイズも取得）
fn get_text_info(layer: usize, frame: usize, section: &ReadSection) -> (String, String, f64)
// → (テキスト, フォント名, サイズ)
```

Lua側では新旧APIの存在確認付きで呼び出し:
```lua
if mod.get_text_info then
    text, obj_fn, obj_sz = mod.get_text_info(layer - 1, frame)
elseif mod.get_text then
    text = mod.get_text(layer - 1, frame)
end
base_fn = (obj_fn ~= "" and obj_fn) or 基準フォント
base_sz = (obj_sz > 0 and obj_sz) or 基準サイズ
```

ビルド: `cargo build --release --target x86_64-pc-windows-msvc` → `.dll` → リネーム `.mod2` → `%ProgramData%\aviutl2\Script\`

## 制御文字のグループ化

同じスタイル（フォント/サイズ/色/回転/変形/オフセット）が連続する文字は、一つの制御文字ブロックで囲むことでバッチ化。
スタイルが変わる位置でのみ制御文字を挿入。

## 使い方

### 基本

1. 空オブジェクトにエフェクトを適用
2. 「テキスト」欄に文字を入力（または `読取レイヤー` でテキストオブジェクト参照）
3. 各種別グループを開き「有効」にチェック、スタイルを設定

### テキストオブジェクト連携

1. テキストオブジェクトを作成
2. 同じオブジェクトにエフェクトを適用（`読取レイヤー=0` 自動）
3. mod2がテキストオブジェクトの文字列＋フォント＋サイズを読取り
4. 各種別すべて無効（デフォルト）なら元のまま描画

## 既知の制限と注意点

### テキストオブジェクトでの obj.load("text") 禁止

AviUtl2仕様上、テキストオブジェクトに適用されたスクリプト内で `obj.load("text",...)` は使用不可。
このスクリプトは空オブジェクト等に適用し、テキストはmod2経由で読む設計。

### 個別オブジェクト非対応

文字ごとに独立したオブジェクトとしての描画（ExEditの個別オブジェクト）には非対応。
`obj.load("text")` 1回で全文字を描画する方式のため。

### 回転の制限

`<tr角度>` は文字の中心を軸に回転する。文字ごとに回転軸を変えることは不可。
（ExEditの個別オブジェクトなら可能だが、上記の制限あり）

### mod2のReadSection

`find_object_after(layer, frame)` はフレーム指定の検索。同一レイヤーに複数オブジェクトがある場合、
目的のオブジェクトを特定できない可能性がある。

## 既知のバグ

- **@範囲指定 で `parse_ranges` 前方参照エラー**: line 537 で `parse_ranges(rng)` を呼んでいるが `function parse_ranges(s)` は line 555 で定義。`grp_cfg` 構築ループ（line 524）が先に実行されるため常に nil。→ `parse_ranges` と `in_ranges` をループより上に移動する必要あり。

## 高速化計画
- 高速化計画: `.agents/plans/optimization.md` TC-1〜TC-5
  - TC-1: テキスト処理結果のフレーム間キャッシュ（静止テキストで95%+削減）
  - TC-2: type_lookupテーブルによるバッチループO(7)→O(1)化
  - TC-3: 文字種判定結果の文字単位メモ化（get_ctype/get_grp）
  - TC-4: text_readerモジュールのロードキャッシュ
  - TC-5: 改行正規化の3→2パス化、共通コード抽出

## 今後の展望

1. **個別オブジェクト対応**: `obj.multiobject` 方式で文字ごとの独立描画を実現
   - 要: テキストオブジェクト制限の回避策（別オブジェクトへの適用）
2. **色・装飾のmod2読み取り**: `get_text_info` に色・縁色・装飾タイプを追加
3. **~~キャッシュ~~** → TC-1（高速化計画に統合）
4. **制御文字エスケープ**: 入力テキストに `<` `>` が含まれる場合の処理
