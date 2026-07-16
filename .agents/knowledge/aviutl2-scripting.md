# AviUtl2 Luaスクリプト作成ガイド

## 基本仕様

| 項目 | 値 |
|------|-----|
| 文字コード | **UTF-8** (BOMなし) |
| 改行コード | CR+LF 推奨 |
| 拡張子 | `.anm2`(アニメーション効果), `.obj2`(カスタムオブジェクト), `.cam2`(カメラ効果), `.scn2`(シーンチェンジ), `.tra2`(トラックバー移動) |
| 配置場所 | `ProgramData\aviutl2\Script\` (および1階層下のサブフォルダ) |
| 実行環境 | LuaJIT 2.1 (64bit) / `--script:lua` で Lua 5.1 切替可 |
| 時間管理 | 秒単位の浮動小数点数 (初代AviUtlはフレーム単位) |
| 利用可能ライブラリ | `table`, `string`, `math` のみ (`os`, `debug`, `ffi.C` は不可) |

## スクリプト構造

### 単一エフェクト

```lua
--track@param1:パラメータ名,0,100,50,0.01
--check@flag:フラグ,false

local val = param1 / 100
if flag then
    obj.alpha = val
end
```

### 複数エフェクト (@name 形式)

ファイル名の先頭を `@` にし、`@名前` で区切る：

```lua
@エフェクトA
--track@value:値,0,100,0,1
obj.alpha = value / 100

@エフェクトB
--track@speed:速度,0,10,1,0.1
obj.ox = obj.ox + speed * obj.time
```

※ `@` で始まらないファイル名の場合は単一エフェクト扱い

## 設定項目（UI定義）

スクリプトの**先頭**に記述。変数名で直接 Lua から参照可能。

### トラックバー
```lua
--track@変数名:項目名,最小値,最大値,デフォルト値[,移動単位,ゼロ値名称,操作倍率]
--track@vx:X速度,-10,10,0,0.01
```
旧形式 `--track0:...` (0〜のインデックス) も利用可能だが非推奨。その場合 `obj.track0` で参照。

### トラックバーグループ化
```lua
--track@x:X,-100,100,0
--track@y:Y,-100,100,0
--track@z:Z,-100,100,0
--trackgroup@x,y,z:座標グループ
```

### チェックボックス
```lua
--check@変数名:項目名,デフォルト値  -- 0/1 → number型, true/false → boolean型
--check@grav:重力,false
```

### チェックボックス（セクション毎）
```lua
--checksection@変数名:項目名,デフォルト値(true/false),初期値(true/false)
```

### 色設定
```lua
--color@変数名:項目名,デフォルト値  -- nil指定で透明色選択可
--color@col:図形色,0xffffff
```

### リスト選択（ドロップダウン）
```lua
--select@変数名:項目名=デフォルト値,選択肢1=値1,選択肢2=値2
--select@deco:装飾,標準文字=0,影付き文字=1,縁取り文字=3
```

### テキスト設定
```lua
--text@変数名:項目名,デフォルト値          -- 複数行
--string@変数名:項目名,デフォルト値        -- 1行
```

### 変数項目（数値/文字列/配列を自由入力）
```lua
--value@変数名:項目名,デフォルト値
--value@num:数値,0
--value@table:配列,{0,0,0}
```

### ファイル/フォルダ選択
```lua
--file@変数名:項目名
--folder@変数名:項目名
```

### フォント/図形設定
```lua
--font@変数名:項目名,デフォルトフォント名
--figure@変数名:項目名,デフォルト図形名
```

### グループ（折りたたみ）
```lua
--group:グループ名,true  -- true=開いた状態
  ...設定項目...
--group                   -- グループ終端
```

### セパレーター（区切り線）
```lua
--separator:名前
```

### その他メタ情報
```lua
--information:スクリプト名 ver1.00 by 作者    -- 情報表示
--label:加工\サブメニュー                       -- メニュー階層
--filter                                        -- フィルタオブジェクト対応
--require:2005400                               -- 必要な本体バージョン
--script:lua                                    -- LuaJIT/Lua切替 (未指定=LuaJIT)
```

### 旧互換の特殊設定 (.tra 系)
```lua
--param:項目名,初期値     -- 汎用パラメータ (obj.getpoint("param") で取得)
--twopoint                -- 中間点無視設定
--speed:加速初期値,減速初期値
--timecontrol             -- 時間制御編集モード
```

## システム変数

### 座標系
| 変数 | 説明 | 読取専用 |
|------|------|---------|
| `obj.x`, `obj.y`, `obj.z` | 表示基準座標 | ○ |
| `obj.ox`, `obj.oy`, `obj.oz` | 相対座標 | |
| `obj.cx`, `obj.cy`, `obj.cz` | 中心相対座標 | |
| `obj.rx`, `obj.ry`, `obj.rz` | 回転角度 (360.0=1回転) | |
| `obj.sx`, `obj.sy`, `obj.sz` | 拡大率 (1.0=等倍) | |
| `obj.zoom` | 拡大率 (1.0=等倍) | |
| `obj.aspect` | アスペクト比 (-1.0〜1.0) | |

### 画像情報
| 変数 | 説明 | 読取専用 |
|------|------|---------|
| `obj.w`, `obj.h` | 画像サイズ (px) | ○ |
| `obj.screen_w`, `obj.screen_h` | スクリーンサイズ (px) | ○ |

### 時間情報
| 変数 | 説明 | 読取専用 |
|------|------|---------|
| `obj.time` | オブジェクト基準の現在時間 (秒) | ○ |
| `obj.totaltime` | オブジェクトの総時間 (秒) | ○ |
| `obj.frame` | 現在のフレーム番号 (0〜) | ○ |
| `obj.totalframe` | 総フレーム数 | ○ |
| `obj.framerate` | フレームレート | ○ |

### その他
| 変数 | 説明 | 読取専用 |
|------|------|---------|
| `obj.alpha` | 不透明度 (0.0〜1.0) | |
| `obj.layer` | レイヤー番号 | ○ |
| `obj.index` | 複数オブジェクト時の番号 (0〜) | ○ |
| `obj.num` | 複数オブジェクト時の総数 | ○ |
| `obj.id` | オブジェクト固有ID (アプリ起動毎) | ○ |
| `obj.effect_id` | エフェクト固有ID (アプリ起動毎) | ○ |
| `global.xxx` | スクリプト間共有テーブル (値はバイナリセーフ文字列) | |

## 主要関数

### obj.draw([ox,oy,oz,zoom,alpha,rx,ry,rz])
オブジェクトを描画。省略時はスクリプト終了時に自動描画。
`obj.draw()` を明示的に呼ぶと、それ以降のフィルタ効果は実行されない。後続フィルタを実行するには `obj.effect()` (引数無し) を使う。

```lua
obj.draw(10, 20, 0, 1, 0.5)  -- ox=10, oy=20, alpha=0.5
```

### obj.load(type, ...)
オブジェクトの画像を読み込み。

| type | 説明 |
|------|------|
| `"image", file` | 画像ファイル読み込み |
| `"movie", file[, time]` | 動画ファイルの指定時間のフレーム |
| `"text", text[, speed, time, align]` | テキスト描画 |
| `"text.layout", text[, ...]` | テキストサイズ取得 (描画せず w,h 返却) |
| `"figure", name[, color, size, line, round]` | 図形描画 |
| `"framebuffer"[, x,y,w,h]` | フレームバッファ読み込み |
| `"tempbuffer"[, x,y,w,h]` | 仮想バッファ読み込み |
| `"layer", no[, effect]` | 他レイヤーのオブジェクト読み込み |
| `"before"` | 直前オブジェクト読み込み (カスタムオブジェクトのみ) |

```lua
-- テキスト描画
obj.setfont("MS Gothic", 30, 0, 0xffffff, 0x000000)
obj.load("text", "表示する文字列", 0, 0, 4)  -- align=4:中央[中]
obj.draw(0, 0)

-- 画像ファイル
obj.load("image", "C:\\path\\to\\image.png")

-- テキストサイズ取得
local tw, th = obj.load("text.layout", "テキスト", 0, 0, 4)
```

### obj.setfont(name, size[, type, col1, col2, bold, italic, charspacing, linespacing])
テキスト描画のフォント設定。`obj.load("text")` の前に毎回設定が必要。

- type: 0=標準, 1=影付き, 2=影付き(薄), 3=縁取り, 4=縁取り(細), 5=縁取り(太), 6=縁取り(角)
- col1: 文字色 (0xRRGGBB)
- col2: 影/縁色

### obj.drawpoly(...)
任意の四角形で変形描画。複数ポリゴンの一括描画が可能で高速。

```lua
-- 4頂点 + UV座標
local poly = {
    -50, -50, 0,  50, -50, 0,  50, 50, 0,  -50, 50, 0,
    0, 0,  obj.w, 0,  obj.w, obj.h,  0, obj.h
}
obj.drawpoly(poly)

-- 色付き4頂点 (r,g,b,a は 0.0〜1.0)
local poly_color = {
    0, 0, 0,  100, 0, 0,  100, 100, 0,  0, 100, 0,
    1, 0, 0, 1,  0, 1, 0, 1,  0, 0, 1, 1,  1, 1, 1, 1
}
obj.drawpoly(poly_color)
```

### obj.setoption(name, value)
各種オプション設定。

| オプション | 値 | 説明 |
|-----------|-----|------|
| `"drawtarget"` | `"tempbuffer"[,w,h]` / `"framebuffer"` | 描画先切替 |
| `"blend"` | `"none"`, `"add"`, `"sub"`, `"mul"`, `"screen"`, `"overlay"` etc. | 合成モード |
| `"culling"` | 0/1 | 裏面非表示 |
| `"billboard"` | 0〜3 | カメラ方向向き |
| `"sampler"` | `"clip"`, `"clamp"`, `"loop"`, `"mirror"`, `"dot"` | サンプラーモード |
| `"draw_state"` | true/false | フレームバッファ描画済みステータス |

### obj.copybuffer(dst, src)
バッファ間コピー。

```lua
obj.copybuffer("tempbuffer", "object")    -- オブジェクト→仮想バッファ
obj.copybuffer("object", "tempbuffer")    -- 仮想バッファ→オブジェクト
obj.copybuffer("cache:名前", "object")    -- キャッシュバッファに保存
obj.copybuffer("object", "cache:名前")    -- キャッシュから復元
```

**重要**: AviUtl2ではキャッシュバッファ (`cache:xxx`) はフレームを跨いで保持されない。永続化したい場合は `obj.getpixeldata()` / `obj.putpixeldata()` でグローバル変数に保存する。

### obj.clearbuffer(target[, color])
バッファをクリア。

```lua
obj.setoption("drawtarget", "tempbuffer", obj.w, obj.h)
obj.clearbuffer("tempbuffer")  -- 透明でクリア
obj.clearbuffer("tempbuffer", 0xffffff)  -- 白でクリア
```

### obj.pixelshader(name, target, {resources}, {constants}, blend, sampler)
ピクセルシェーダー実行。

```lua
-- 定義: スクリプト先頭の複数行コメント
--[[pixelshader@psmain:
cbuffer constant0 : register(b0) {
    float bright;
};
float4 psmain(float4 pos : SV_Position, float2 uv : TEXCOORD0) : SV_Target {
    // uv は描画範囲が 0.0〜1.0
    ...
    return color;
}
]]

-- 実行
obj.pixelshader("psmain", "object", {"tempbuffer"}, {bright / 100}, "copy", "clamp")
```

### obj.computeshader(name, {targets}, {resources}, {constants}, countX, countY, countZ[, sampler])
コンピュートシェーダー実行。

```lua
-- 定義
--[[computeshader@csmain:
Texture2D<float4> src : register(t0);
RWTexture2D<float4> dst : register(u0);

[numthreads(8, 8, 1)]
void csmain(uint3 id : SV_DispatchThreadID) {
    ...
}
]]

-- 実行
local countX = math.ceil(obj.w / 8)
local countY = math.ceil(obj.h / 8)
obj.computeshader("csmain", {"tempbuffer"}, {"object"}, constants, countX, countY, 1)
```

### obj.effect([name, param1, value1, ...])
フィルタ効果を実行。メディアオブジェクトのみ使用可。

```lua
obj.effect("色調補正", "明るさ", 150, "色相", 180)
obj.effect("単色化", "強さ", 100.0, "輝度を保持する", 1, "color", 0x0000ff)
-- 引数なし = スクリプト以降のフィルタ効果を実行
obj.effect()
```

### obj.getvalue(target[, time, section])
設定値の取得。

```lua
-- トラックバー値 (変数名)
local v = obj.getvalue("track.vx")
-- トラックバー値 (旧インデックス)
local v = obj.getvalue(0)
-- エフェクトの設定値
local font = obj.getvalue("テキスト", "フォント")
local range = obj.getvalue("ぼかし:1", "範囲")  -- 同名エフェクトの2番目
-- 特定時間の値
local px = obj.getvalue("x", 1.5, 0)
```

### obj.getpoint(target[, option])
`.tra2` 用。トラックバーポイントの値や中間点情報を取得。

```lua
local param = obj.getpoint("param")     -- --param で定義した値
local i = obj.getpoint("index")         -- 現在のインデックス
local v = obj.getpoint(i)               -- i番ポイントの値
local a, b = obj.getpoint("link")       -- 前後のリンク情報
```

### obj.multiobject(num, func)
複数オブジェクトの個別描画。

```lua
obj.multiobject(5, function()
    obj.load("text", "文字" .. (obj.index + 1))
    obj.ox = obj.index * 50
end)
```

### obj.getpixeldata() / obj.putpixeldata()
ピクセルデータの高速な取得・書き込み。フレーム間でのバッファ永続化に使う。

```lua
local data, w, h = obj.getpixeldata("cache:orig", "rgba")
-- グローバル変数に保存
_G.saved_data = data

-- 復元
obj.setoption("drawtarget", "tempbuffer", w, h)
obj.putpixeldata("tempbuffer", saved_data, w, h, "rgba")
```

### obj.module("モジュール名")
`.mod2` モジュールの読み込み。

```lua
local mod = obj.module("Basic_S")
-- or
local mod = require("Basic_S")  -- AviUtl2では require が使える
```

### obj.getinfo(name, ...)
情報取得。

```lua
local script_time = obj.getinfo("script_time")  -- スクリプト処理時間(ms)
local is_filter = obj.getinfo("filter")          -- フィルタオブジェクト判定
```

### obj.rand(st, ed[, seed, frame]) / obj.rand1([seed, frame])
決定論的乱数。同一フレームで常に同じ値を返す。

```lua
local r = obj.rand(0, 100, obj.index, obj.frame)  -- 0〜100の整数
local r1 = obj.rand1()  -- 0.0以上1.0未満
```

## バッファシステム

| バッファ名 | 説明 |
|-----------|------|
| `"object"` | 現在のオブジェクト画像 |
| `"tempbuffer"` | 仮想バッファ（全オブジェクト共用、書き込まないと不定） |
| `"framebuffer"` | フレームバッファ（最終的な画面出力先） |
| `"cache:名前"` | キャッシュバッファ（フレーム内でのみ保持、次フレームで消える） |

```
描画の流れ:
1. obj.setoption("drawtarget", "tempbuffer", w, h) -- 描画先を仮想バッファに
2. obj.clearbuffer("tempbuffer")                   -- クリア
3. obj.copybuffer("object", "元データ")             -- 元画像を読み込み
4. obj.draw() / obj.drawpoly()                     -- 仮想バッファに描画
5. obj.setoption("drawtarget", "framebuffer")      -- 描画先をフレームバッファに戻す
6. obj.copybuffer("object", "tempbuffer")          -- 結果をオブジェクトに戻す
```

## レシピ集

### 基本: 画像にテキストを重ねる
```lua
--track@txt:テキスト,デフォルト文字列
--font@font:フォント,MS UI Gothic
--track@size:サイズ,10,200,30,1
--color@txtcol:文字色,0xffffff
--track@x:位置X,-1000,1000,0,1
--track@y:位置Y,-1000,1000,0,1

obj.copybuffer("tempbuffer", "object")
obj.setoption("drawtarget", "tempbuffer")

obj.setfont(font, size, 0, txtcol, 0x000000)
obj.load("text", txt)
obj.draw(x, y)

obj.setoption("drawtarget", "framebuffer")
obj.copybuffer("object", "tempbuffer")
```

### 基本: ピクセルシェーダーで色調補正
```lua
--track@bright:明るさ,-100,100,0,0.1
--[[pixelshader@psmain:
cbuffer cb : register(b0) {
    float bright;
};
float4 psmain(float4 pos : SV_Position, float2 uv : TEXCOORD0) : SV_Target {
    return float4(bright, bright, bright, 1);
}
]]
obj.pixelshader("psmain", "object", nil, {bright / 100}, "add", "clamp")
```

### 上級: コンピュートシェーダー + ピクセルシェーダーの二段処理
パターン（`@DataEffect_Su.anm2` 参照）:
1. `obj.copybuffer` で元画像を `cache:` に退避
2. `obj.computeshader` で計算 → `cache:` に出力
3. `obj.pixelshader` で最終描画

## 開発上の注意点

1. **構文チェック**: `luac -p filename.anm2` で最低限の文法確認
2. **キャッシュ破棄**: スクリプト修正後は AviUtl2 で「キャッシュを破棄」(F5)
3. **グローバル変数**: スクリプト間で状態を共有する場合は `_G.名前 = {}` パターン。`obj.effect_id` でエフェクト単位の識別をする
4. **local 推奨**: グローバル名前空間汚染を防ぐため、変数は `local` で宣言
5. **パラメータ名**: ユーザー向け項目名は日本語（半角カナで簡潔に）
6. **色の分解**:
   ```lua
   local r = math.floor(color / 0x10000) / 255.0
   local g = math.floor((color % 0x10000) / 0x100) / 255.0
   local b = (color % 0x100) / 255.0
   ```
7. **テキスト描画後に obj.cx, obj.cy が上書きされる**: テキスト描画前に退避し、後に復元する
   ```lua
   local orig_cx, orig_cy = obj.cx, obj.cy
   obj.load("text", "...")
   obj.draw(x, y)
   obj.cx, obj.cy = orig_cx, orig_cy
   ```

## 参照リポジトリ

実際の `.anm2` スクリプトを読むのが最も実践的な学習方法。

- **Suzuke本人のスクリプト**: `/mnt/c/AviUtl2/data/Script/Suzuke/`
  - `@DataEffect_Su.anm2` — PS + CS の二段処理、テキスト表示、tracking の完全な実装例
  - `@Glitch_Su.anm2` — CS による Bitonic sort の実装例
  - `@入退場.anm2` — シンプルなアニメーション効果の入門例
  - `@情報取得_Su.anm2` — テキスト情報表示のシンプル例
  - `データモッシュ_Su.obj2` — 旧スクリプト移植のパターン例

- **他の作者のスクリプト**: `/mnt/c/AviUtl2/data/Script/`
  - `@Basic_S.anm2` — 大規模で高品質なスクリプト集 (sigma-axis 作)
  - `@Basic_S.tra2` — `.tra2` の実装例
