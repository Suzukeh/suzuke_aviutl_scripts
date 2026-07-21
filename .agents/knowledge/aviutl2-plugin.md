# AviUtl2 プラグイン開発ガイド (Rust)

> 最終更新: 2026-07-17

## 1. プラグインの種類と配置場所

| 拡張子 | 種類 | 配置場所 | 説明 |
|--------|------|----------|------|
| `.aui2` | 入力プラグイン | `%ProgramData%\aviutl2\Plugin\` | 他ファイル形式の読み込み |
| `.auo2` | 出力プラグイン | `%ProgramData%\aviutl2\Plugin\` | 他ファイル形式への出力 |
| `.auf2` | フィルタプラグイン | `%ProgramData%\aviutl2\Plugin\` | 画像/音声フィルタ効果、カスタムオブジェクト |
| `.auc2` | 色変換プラグイン | `%ProgramData%\aviutl2\Plugin\` | 色変換 |
| `.aux2` | 汎用プラグイン | `%ProgramData%\aviutl2\Plugin\` | 独自ウィンドウ、メニュー追加、プロジェクト編集 |
| `.aul2` | 言語拡張リソース | `%ProgramData%\aviutl2\Plugin\` | 多言語翻訳ファイル |
| `.mod2` | スクリプトモジュール | `%ProgramData%\aviutl2\Script\` | Luaから `obj.module()` / `require()` で呼ぶDLL |

## 2. Rust SDK: aviutl2-rs

- **リポジトリ**: [sevenc-nanashi/aviutl2-rs](https://github.com/sevenc-nanashi/aviutl2-rs)
- **最新バージョン**: v0.39.0 (2026/7/12)
- **ライセンス**: MIT
- **ターゲット**: `x86_64-pc-windows-msvc` (DLL)
- **紹介動画**: [sm45355531](https://www.nicovideo.jp/watch/sm45355531)

### 2.1 クレート構成

| クレート | crates.io | 役割 |
|----------|-----------|------|
| `aviutl2` | [crates.io/crates/aviutl2](https://crates.io/crates/aviutl2) | **高レベルAPI**（これを依存に追加する） |
| `aviutl2-sys` | [crates.io/crates/aviutl2-sys](https://crates.io/crates/aviutl2-sys) | CヘッダのFFIバインディング |
| `aviutl2-macros` | [crates.io/crates/aviutl2-macros](https://crates.io/crates/aviutl2-macros) | deriveマクロ、登録マクロ |
| `aviutl2-eframe` | [crates.io/crates/aviutl2-eframe](https://crates.io/crates/aviutl2-eframe) | 汎用プラグイン向けegui UIラッパー |
| `aviutl2-alias` | [crates.io/crates/aviutl2-alias](https://crates.io/crates/aviutl2-alias) | `*.aup2`, `*.object`, `*.effect` の読み書き |

### 2.2 Feature Flags

| Feature | デフォルト | 内容 |
|---------|-----------|------|
| `input` | on | 入力プラグイン |
| `output` | on | 出力プラグイン |
| `filter` | on | フィルタプラグイン |
| `module` | on | スクリプトモジュール |
| `generic` | on | 汎用プラグイン |
| `wrap_log` | on | ログ出力の自動改行 |
| `aviutl2-alias` | on | エイリアスファイル読み書き |
| `image` | off | `image` クレート連携 |
| `serde` | off | プロジェクトファイルへのデータ保存 |

Cargo.tomlで不要なfeatureを切ればバイナリサイズを減らせる:
```toml
[dependencies]
aviutl2 = { version = "0.39", default-features = false, features = ["filter"] }
```

## 3. 全プラグイン種別の実装テンプレート

### 3.1 共通: `#[aviutl2::plugin]` 属性

構造体に `#[aviutl2::plugin(FilterPlugin)]` のように付けることで、
DLLのエクスポート関数 (`InitializePlugin`, `GetFilterPluginTable` 等) を自動生成する。

指定できる値:
- `InputPlugin`
- `OutputPlugin`
- `FilterPlugin`
- `ScriptModule`
- `GenericPlugin`

### 3.2 フィルタプラグイン (.auf2)

**トレイト**: `FilterPlugin`
**登録マクロ**: `register_filter_plugin!`
**主要メソッド**:

| メソッド | 説明 |
|----------|------|
| `fn new(info: AviUtl2Info) -> AnyResult<Self>` | 初期化 |
| `fn plugin_info(&self) -> FilterPluginTable` | プラグイン情報返却 |
| `fn proc_video(&self, config, video) -> AnyResult<()>` | 画像フィルタ処理 |
| `fn proc_audio(&self, config, audio) -> AnyResult<()>` | 音声フィルタ処理 |
| `fn before_proc_video(...)` / `fn after_proc_video(...)` | 前後処理 |
| `fn is_save_frame(...)` | フレーム保存可否 |
| `fn project_save/load(...)` | プロジェクト保存/復元 (要 `serde` feature) |

**FilterPluginTable 構造体**:
```rust
FilterPluginTable {
    name: "表示名".to_string(),
    label: Some("加工\\サブメニュー".to_string()), // メニュー階層
    information: "バージョン情報等".to_string(),
    flags: aviutl2::bitflag!(FilterPluginFlags {
        video: true,        // 画像フィルタ
        audio: false,       // 音声フィルタ
        filter: true,       // フィルタオブジェクト対応
        single: false,      // 単独使用(カスタムオブジェクト)
        video_scene: false, // シーンの映像レンダリング
        render: false,      // 非対応
        resize: false,      // リサイズ
        require: false,     // 必須フィルタ
        always_active: false, // 常にアクティブ
        save_frame: false,  // フレーム保存
    }),
    config_items: Config::to_config_items(),
}
```

**FilterPluginFlags 全フィールド** (bool):
- `video`: 映像フィルタとして動作
- `audio`: 音声フィルタとして動作
- `filter`: フィルタオブジェクトで使用可能
- `single`: 単独で使用（カスタムオブジェクト的）
- `video_scene`: シーンの映像レンダリング
- `render`: (非対応)
- `resize`: リサイズ可能
- `require`: このフィルタが必須（常に存在）
- `always_active`: 常にアクティブ
- `save_frame`: フレーム保存用

**設定項目の定義** (deriveマクロを使用):

```rust
// ドロップダウン選択肢の定義
#[derive(Debug, Clone, PartialEq, Eq, FilterConfigSelectItems)]
enum SortDirection {
    #[item(name = "左右")]
    Horizontal,
    #[item(name = "左右（反転）")]
    HorizontalInverted,
    #[item(name = "上下")]
    Vertical,
}

// 設定項目構造体
#[aviutl2::filter::filter_config_items]
#[derive(Debug, Clone, PartialEq)]
struct Config {
    // トラックバー
    #[track(name = "しきい値", range = 0.0..=1.0, step = 0.001, default = 0.5)]
    threshold: f64,

    // ドロップダウン
    #[select(name = "方向", items = SortDirection, default = SortDirection::Horizontal)]
    direction: SortDirection,

    // チェックボックス
    #[checkbox(name = "有効", default = true)]
    enabled: bool,

    // 色選択
    #[color(name = "色", default = 0xffffff)]
    color: u32,

    // 文字列入力 (1行)
    #[string(name = "名前", default = "")]
    name: String,

    // テキスト (複数行)
    #[text(name = "説明", default = "")]
    description: String,

    // ファイル選択
    #[file(name = "ファイル")]
    file: Option<String>,

    // フォルダ選択
    #[folder(name = "フォルダ")]
    folder: Option<String>,

    // ボタン
    #[button(name = "実行", button = "実行する")]
    execute: (),
}

// proc_video内での設定値取得
let cfg: Config = config.to_struct();
```

**フィルタプラグイン完全な例**:

```rust
use aviutl2::{AnyResult, AviUtl2Info, filter::*};

#[aviutl2::plugin(FilterPlugin)]
struct MyFilter;

impl FilterPlugin for MyFilter {
    fn new(_info: AviUtl2Info) -> AnyResult<Self> {
        // ログ初期化 (任意)
        aviutl2::tracing_subscriber::fmt()
            .with_max_level(aviutl2::tracing::Level::INFO)
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .init();
        Ok(Self)
    }

    fn plugin_info(&self) -> FilterPluginTable { /* ... */ }

    fn proc_video(
        &self,
        config: &[FilterConfigItem],
        video: &mut FilterProcVideo,
    ) -> AnyResult<()> {
        let cfg: Config = config.to_struct();
        let w = video.video_object.width as usize;
        let h = video.video_object.height as usize;
        let mut pixels = vec![RgbaPixel::default(); w * h];
        video.get_image_data(&mut pixels);

        // ピクセル加工...

        video.set_image_data(&pixels, video.video_object.width, video.video_object.height);
        Ok(())
    }
}

aviutl2::register_filter_plugin!(MyFilter);
```

**FilterProcVideo の主要メソッド**:
- `video.get_image_data(&mut [RgbaPixel])` — 画像データ取得
- `video.set_image_data(&[RgbaPixel], w, h)` — 画像データ書き込み
- `video.video_object` — `VideoObjectInfo` (width, height, fps, ...)
- `video.object_info` — `ObjectInfo` (flag, frame, ...)
- `video.scene_info` — `SceneInfo` (w, h, ...)
- `video.draw_image(param)` — 別画像の描画
- `video.pixel_shader(...)` — ピクセルシェーダー実行
- `video.compute_shader(...)` — コンピュートシェーダー実行

### 3.3 スクリプトモジュール (.mod2)

**トレイト**: `ScriptModule`
**登録マクロ**: `register_script_module!`

**最小実装**:
```rust
use aviutl2::{AnyResult, module::ScriptModuleFunctions};

#[aviutl2::plugin(ScriptModule)]
struct MyModule;

impl aviutl2::module::ScriptModule for MyModule {
    fn new(_info: aviutl2::AviUtl2Info) -> AnyResult<Self> {
        Ok(Self)
    }

    fn plugin_info(&self) -> aviutl2::module::ScriptModuleTable {
        aviutl2::module::ScriptModuleTable {
            information: "My Module v1.0 by Suzuke".to_string(),
            functions: Self::functions(),
        }
    }
}

// 関数定義: 全メソッドが Lua から呼べる関数になる
#[aviutl2::module::functions]
impl MyModule {
    fn add(&self, a: i32, b: i32) -> AnyResult<i32> {
        Ok(a + b)
    }

    fn greet(&self, name: String) -> AnyResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

aviutl2::register_script_module!(MyModule);
```

Luaからの呼び出し:
```lua
local mod = obj.module("my_module")
print(mod.add(1, 2))       -- 3
print(mod.greet("World"))  -- "Hello, World!"
```

**引数として受け取れる型** (`FromScriptModuleParam` を実装):
- `i32`, `f64`, `bool`, `String`
- `Vec<T>`, `HashMap<String, T>`
- `ScriptModuleUserData<T>` (ユーザーデータ)
- カスタム型 (`#[derive(FromScriptModuleParam)]`)
- `ScriptModuleCallHandle` (生の引数ハンドル)

**戻り値として使える型** (`IntoScriptModuleReturnValue` を実装):
- `i32`, `f64`, `bool`, `String`
- `Vec<T>`, タプル `(T1, T2, ...)`
- `ScriptModuleUserData<T>`
- `ScriptModuleReturnValue`

**ユーザーデータ (Luaのメタテーブル付きオブジェクト)**:
```rust
#[derive(Debug, Clone)]
struct MyUserData {
    value: i32,
}

// Lua 側で obj.foo のようにアクセス可能に
#[aviutl2::module::metatable]
impl MyUserData {
    fn index(&self, _this: (), key: String) -> AnyResult<Option<i32>> {
        match key.as_str() {
            "value" => Ok(Some(self.value)),
            _ => Ok(None),
        }
    }
}

// 関数として呼び出し可能に (obj("arg"))
#[aviutl2::module::metatable]
impl MyCallback {
    fn call(&self, _this: (), arg: String) -> AnyResult<String> {
        Ok(format!("called with {}", arg))
    }
}

// 関数内でユーザーデータを返す
#[aviutl2::module::functions]
impl MyModule {
    fn create_data(&self, v: i32) -> AnyResult<ScriptModuleUserData<MyUserData>> {
        Ok(ScriptModuleUserData::new(MyUserData { value: v }))
    }
}
```

### 3.4 入力プラグイン (.aui2)

**トレイト**: `InputPlugin`
**登録マクロ**: `register_input_plugin!`
**Associated Type**: `InputHandle` (開いたファイルの状態)

**主要メソッド**:
| メソッド | 説明 |
|----------|------|
| `fn new(info) -> AnyResult<Self>` | 初期化 |
| `fn plugin_info(&self) -> InputPluginTable` | 情報返却 |
| `fn open(&self, path) -> AnyResult<Self::InputHandle>` | ファイルを開く |
| `fn get_input_info(&self, handle, vt, at) -> AnyResult<InputInfo>` | 映像/音声情報 |
| `fn read_video_mut(&self, handle, frame, returner) -> AnyResult<()>` | フレーム読み出し |
| `fn read_audio_mut(&self, handle, start, length, returner) -> AnyResult<()>` | 音声読み出し |
| `fn time_to_frame(&self, handle, track, time) -> AnyResult<u32>` | 時間→フレーム変換 |
| `fn is_keyframe(&self, handle, frame) -> AnyResult<bool>` | キーフレーム判定 |
| `fn close(&self, handle) -> AnyResult<()>` | クローズ |

**InputPluginTable**:
```rust
InputPluginTable {
    name: "表示名".to_string(),
    input_type: InputType::Video,  // Video / Audio / Both
    file_filters: aviutl2::file_filters! {
        "画像ファイル" => ["png", "jpg", "bmp"],
        "全てのファイル" => ["*"],
    },
    information: "...".to_string(),
    can_config: false,  // 設定ダイアログの有無
    concurrent: false,  // 同時使用可否
}
```

**ピクセルフォーマット** (`InputPixelFormat`):
- `Bgra` — 8bit BGRA
- `Pa64` — 16bit RGBA (premultiplied alpha)

### 3.5 出力プラグイン (.auo2)

**トレイト**: `OutputPlugin`
**登録マクロ**: `register_output_plugin!`

**主要メソッド**:
| メソッド | 説明 |
|----------|------|
| `fn new(info) -> AnyResult<Self>` | 初期化 |
| `fn plugin_info(&self) -> OutputPluginTable` | 情報 |
| `fn init_output(&self, output_info) -> AnyResult<Self::OH>` | 出力開始 |
| `fn save(&self, handle, frame, returner) -> AnyResult<()>` | フレーム保存 |
| `fn save_finish(&self, handle) -> AnyResult<()>` | 出力終了処理 |
| `fn get_output_plugin_table(&self) -> OutputPluginTable` | 情報取得 |

### 3.6 汎用プラグイン (.aux2)

**トレイト**: `GenericPlugin`
**登録マクロ**: `register_generic_plugin!`

できること:
- メニュー項目の追加 (`#[aviutl2::generic::menus]`)
- `EditHandle` でプロジェクトの編集操作
- タイムラインのオブジェクト操作
- `ProjectFile` でプロジェクトへのデータ保存
- ウィンドウ表示 (egui: `aviutl2-eframe`)

**注意**: 汎用プラグインは `plugin2.h` 相当（紛らわしいので `generic` と命名されている）。

## 4. 設定項目の全種別

`#[aviutl2::filter::filter_config_items]` で使えるフィールド属性の一覧:

| 属性 | 型 | 説明 |
|------|-----|------|
| `#[track(name, range, step?, default, ...)]` | `f64` | トラックバー (range は `0.0..=1.0` 形式) |
| `#[select(name, items, default)]` | enum | ドロップダウン選択 |
| `#[checkbox(name, default)]` | `bool` | チェックボックス |
| `#[checksection(name, default, init?)]` | `bool` | セクション毎チェックボックス |
| `#[color(name, default)]` | `u32` | 色選択 (0xRRGGBB) |
| `#[string(name, default)]` | `String` | 1行文字列 |
| `#[text(name, default)]` | `String` | 複数行テキスト |
| `#[file(name)]` | `Option<String>` | ファイル選択 |
| `#[folder(name)]` | `Option<String>` | フォルダ選択 |
| `#[button(name, button)]` | `()` | ボタン |
| `#[data(name)]` | `FilterConfigDataHandle` | 汎用データ |
| `#[group(name, default_open?)]` | `()` | グループ開始 |
| `#[separator(name)]` | `()` | セパレーター |

## 5. プロジェクト構成

```
my-plugin/
├── Cargo.toml
├── aviutl2.toml           # au2 CLI用ビルド設定
└── src/
    └── lib.rs
```

**Cargo.toml**:
```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]   # DLLとしてビルド

[dependencies]
aviutl2 = "0.39"
```

**aviutl2.toml**:
```toml
[project]
id = "suzuke.my-plugin"
name = "My Plugin"
version = "0.1.0"

[development]
aviutl2_version = "latest"

[artifacts.my_plugin]
destination = "Plugin/my_plugin.auf2"

[artifacts.my_plugin.profiles.debug]
source = "target/debug/my_plugin.dll"

[artifacts.my_plugin.profiles.release]
source = "target/release/my_plugin.dll"
```

## 6. ビルドと配置

### 6.1 au2 CLIを使う方法（推奨）

```bash
# インストール
cargo install aviutl2-cli

# プロジェクト初期化
au2 init

# AviUtl2本体のダウンロード + 開発環境セットアップ
au2 prepare

# ビルド + シンボリックリンク配置
au2 dev

# リリースパッケージ作成
au2 release
```

### 6.2 手動

```bash
cargo build --release --target x86_64-pc-windows-msvc
# target/release/my_plugin.dll
# → %ProgramData%\aviutl2\Plugin\my_plugin.auf2 として配置
# → %ProgramData%\aviutl2\Script\my_module.mod2 として配置
```

## 7. ログ出力

```rust
// 初期化 (FilterPlugin::new などで一度だけ)
aviutl2::tracing_subscriber::fmt()
    .with_max_level(aviutl2::tracing::Level::DEBUG)
    .event_format(aviutl2::logger::AviUtl2Formatter)
    .with_writer(aviutl2::logger::AviUtl2LogWriter)
    .init();

// 使用 (トレースレベルで出力)
tracing::debug!("デバッグ情報: {:?}", value);
tracing::info!("情報: {}", msg);
tracing::warn!("警告");
tracing::error!("エラー");

// 簡易マクロ
aviutl2::lprintln!("Hello");  // println! 相当
aviutl2::ldbg!(&value);       // dbg! 相当
```

## 8. プラグイン vs Luaスクリプトの選択基準

| ケース | 推奨 | 理由 |
|--------|------|------|
| 簡単な座標計算、テキスト制御 | Luaスクリプト | シンプル |
| 画像ピクセル処理（HLSLで済む） | Lua + シェーダー | GPUで高速 |
| 画像ピクセル処理（複雑なCPUロジック） | `.auf2` フィルタ | Rustの速度 |
| Luaの関数群を高速化したい | `.mod2` | DLLで高速化 |
| 外部ファイル読み書き | `.aui2` / `.auo2` | 専用設計 |
| 独自ウィンドウ、メニュー追加 | `.aux2` | 唯一の手段 |
| エコシステム(crates.io)活用 | Rust | 豊富なライブラリ |
| 既存C++コードの移植 | C++ | 書き直し不要 |

## 9. 実装上の注意点

1. **パニック**: RustのpanicはFFI境界でUBになる。`register_*_plugin!` マクロはデフォルトでcatch_unwindするが、意図的に `unwind = true/false` を指定可能。
2. **スレッド安全性**: コールバックは複数スレッドから呼ばれる可能性がある。共有状態には `Mutex`, `RwLock`, `Arc` を使う。
3. **DLL名**: `crate-type = ["cdylib"]` でビルドした `.dll` のファイル名がそのままプラグイン名になる（ただし拡張子は変更）。
4. **マクロの内部アイテム**: `__` で始まるAPIは内部実装用。semver保証外なので使わない。
5. **AviUtl2 SDKの更新**: SDKが頻繁に更新されるため、`aviutl2-rs` のAPIも破壊的変更がありうる。CHANGELOGを確認すること。

## 10. 参照サンプル一覧

| サンプル | 種別 | 説明 |
|----------|------|------|
| [pixelsort-filter](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/pixelsort-filter) | `.auf2` | ピクセルソートフィルタ (SIMD/Rayon対応) |
| [random-color-filter](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/random-color-filter) | `.auf2` | ランダム色表示カスタムオブジェクト |
| [binaural-filter](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/binaural-filter) | `.auf2` | バイノーラル音声パン振り |
| [equalizer-filter](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/equalizer-filter) | `.auf2` | イコライザーフィルタ |
| [chiptune-filter](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/chiptune-filter) | `.auf2` | チップチューン音源カスタムオブジェクト |
| [regex-module](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/regex-module) | `.mod2` | 正規表現モジュール（ユーザーデータ活用例） |
| [username-module](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/username-module) | `.mod2` | 最小構成のモジュール |
| [image-rs-input](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/image-rs-input) | `.aui2` | image-rsで画像読み込み |
| [image-rs-output](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/image-rs-output) | `.auo2` | image-rsで画像書き出し |
| [ffmpeg-output](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/ffmpeg-output) | `.auo2` | FFmpegで動画出力 |
| [metronome-plugin](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/metronome-plugin) | `.aux2` | メトロノーム (egui UI) |
| [srt-file-plugin](https://github.com/sevenc-nanashi/aviutl2-rs/tree/main/examples/srt-file-plugin) | `.aux2` | SRT字幕インポート/エクスポート |

## 11. スターターキットを作る場合の最小構成

```bash
cargo init --lib my-aviutl2-plugin
cd my-aviutl2-plugin
```

`Cargo.toml`:
```toml
[package]
name = "my-aviutl2-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
aviutl2 = "0.39"
```

`src/lib.rs`:
```rust
use aviutl2::{AnyResult, AviUtl2Info, filter::*};

#[aviutl2::plugin(FilterPlugin)]
struct MyPlugin;

impl FilterPlugin for MyPlugin {
    fn new(_info: AviUtl2Info) -> AnyResult<Self> { Ok(Self) }
    fn plugin_info(&self) -> FilterPluginTable {
        FilterPluginTable {
            name: "My Plugin".into(),
            label: None,
            information: "v0.1.0 by Suzuke".into(),
            flags: aviutl2::bitflag!(FilterPluginFlags { video: true, filter: true }),
            config_items: vec![],
        }
    }
    fn proc_video(&self, _config: &[FilterConfigItem], video: &mut FilterProcVideo) -> AnyResult<()> {
        let w = video.video_object.width as usize;
        let h = video.video_object.height as usize;
        let mut pixels = vec![RgbaPixel::default(); w * h];
        video.get_image_data(&mut pixels);
        // ここで加工
        video.set_image_data(&pixels, video.video_object.width, video.video_object.height);
        Ok(())
    }
}

aviutl2::register_filter_plugin!(MyPlugin);
```
