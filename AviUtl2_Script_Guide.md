# AviUtl2 スクリプト開発ガイド (初代AviUtlとの違いと新機能)

AviUtl2（拡張編集2）におけるLuaスクリプトの仕様、初代AviUtl（ExEdit）との違い、および新機能についてまとめた技術ドキュメントです。

---

## 1. 基本仕様と環境の違い

| 項目 | 初代AviUtl (ExEdit) | AviUtl2 |
| :--- | :--- | :--- |
| **文字コード** | Shift_JIS (Windows-31J) | **UTF-8** |
| **改行コード** | CR+LF 推奨 | CR+LF 推奨 |
| **拡張子** | `.anm`, `.obj`, `.cam`, `.scn`, `.tra` | **`.anm2`**, **`.obj2`**, **`.cam2`**, **`.scn2`**, **`.tra2`** |
| **配置場所** | `aviutl/script/` 以下の任意のフォルダ | `ProgramData\aviutl2\Script\` フォルダ (および1つ下のフォルダ) |
| **実行環境** | Lua 5.1 (32bit) | **LuaJIT** (2.1版 64bit) / `--script:lua` でLua 5.1切替可能 |
| **時間管理** | フレーム単位管理 | **秒数（時間）単位管理** (浮動小数点数) |
| **互換性** | - | 旧形式ファイルも読み込めますが、一部機能（pixel入出力の引数等）が非対応。32bit DLL依存のスクリプトは動作しません。 |

---

## 2. 設定項目（UI定義）の新書式

従来の `--track0` などのインデックス指定に代わり、**直接変数名を指定する新しい定義書式**が導入されました。

### トラックバー
*   **新書式**: `--track@変数名:項目名,最小値,最大値,デフォルト値[,移動単位,ゼロ値名称,操作倍率]`
*   **例**: `--track@vx:X速度,-10,10,0`
*   ※ 従来の `track0` の代わりに、スクリプト内からは定義した `vx` という変数名で直接値を参照できます（`obj.track0` でも参照可能）。
*   **ゼロ値名称**: 値が0の時にスライダーに表示する別名（例: `0` の代わりに `なし` 等）を設定可能。
*   **トラックバーのグループ化**: `--trackgroup@x,y,z:グループ名` のように記述することで、複数のトラックバーを一行にグループ化して表示できます。

### チェックボックス
*   **新書式**: `--check@変数名:項目名,デフォルト値(0か1、またはtrueかfalse)`
*   **例**: `--check@grav:重力,false` (boolean型として動作)
*   **セクション毎設定**: `--checksection@変数名:項目名,デフォルト値,初期値` が追加されました。

### 新しく追加されたUI要素
*   **リスト選択（ドロップダウン）**:
    `--select@変数名:項目名=デフォルト値,選択肢1=値1,選択肢2=値2`
*   **テキスト設定（複数行）**:
    `--text@変数名:項目名,デフォルト値`
*   **文字列設定（1行）**:
    `--string@変数名:項目名,デフォルト値`
*   **変数項目（数値、文字列、テーブル(配列)を自由に入力可能）**:
    `--value@変数名:項目名,デフォルト値`
*   **フォルダ選択**:
    `--folder@変数名:項目名`
*   **設定グループ化（折りたたみ）**:
    `--group:グループ名,デフォルト表示状態(true/false)` 〜 `--group`（終端）でUI項目をグループ化可能。
*   **セパレーター（区切り線）**:
    `--separator:セパレーター名`

---

## 3. GPUシェーダー（HLSL）の直接実行

AviUtl2の最大の追加機能の一つが、**スクリプト内でのDirect3D11シェーダーの直接記述と実行**です。

### ピクセルシェーダーの定義と実行
スクリプトの先頭で複数行コメントを使用してHLSLを記述します。

```lua
--[[pixelshader@psmain:
cbuffer constant0 : register(b0) {
    float bright;
};
float4 psmain(float4 pos : SV_Position) : SV_Target {
    return float4(bright, bright, bright, 1);
}
]]

-- Lua側での呼び出し
-- obj.pixelshader(登録名, 出力先バッファ, 参照バッファ配列, 定数配列, ブレンドモード, サンプラー)
obj.pixelshader("psmain", "object", nil, {bright / 100}, "add")
```

### コンピュートシェーダーの定義と実行
GPUを用いた高度な並列計算や画像処理が可能です。

```lua
--[[computeshader@csmain:
[numthreads(8, 8, 1)]
void csmain(uint3 id : SV_DispatchThreadID) {
    // 処理
}
]]

-- Lua側での呼び出し
-- obj.computeshader(登録名, 出力バッファリスト, 入力リソースリスト, 定数配列, Xスレッド数, Yスレッド数, Zスレッド数, サンプラー)
obj.computeshader("csmain", {"tempbuffer"}, {"object"}, constants, countX, countY, 1)
```

---

## 4. 主な新規変数・新規関数

### 新しい変数
*   **`obj.id`** / **`obj.effect_id`**: アプリ起動毎に一意に割り当てられるオブジェクト・エフェクトの固有ID。
*   **`obj.sx` / `obj.sy` / `obj.sz`**: 基準座標からの拡大率。
*   **`global.xxx`**: スクリプト間でバイナリセーフな文字列としてデータを共有できる共通テーブル変数。

### 特筆すべき新関数
*   **`obj.multiobject(num, func)`**
    オブジェクトを個別オブジェクトとして複数回描画するコールバック関数仕様。
    従来のループ処理による個別描画に比べ、より最適化された制御が可能です。
    ```lua
    local text = {"あ", "い", "う", "え", "お"}
    local ox = 0
    obj.multiobject(#text, function()
        obj.load("text", text[obj.index + 1])
        obj.ox = ox
        ox = ox + obj.w
    end)
    ```
*   **`obj.getvalue(layer, effect, item, ...)`**
    他のレイヤーに配置されているオブジェクトや、特定のエフェクトの設定値を高度に取得することができます。
*   **`obj.load("text.layout", text, ...)`** (または `"textlayout"`)
    テキストを描画した際のサイズ（w, h）や中心座標（cx, cy）を描画することなく取得できます。
*   **`obj.module("モジュール名")`**
    拡張子 `.mod2` のスクリプトモジュールを読み込み、関数テーブルとして利用できます。
*   **`obj.getpixeldata()` / `obj.putpixeldata()`**
    外部DLLやC言語系モジュールとの間で画像ピクセルデータを高速にやり取りするためのユーザーデータポインタを取得・書き込みします。
