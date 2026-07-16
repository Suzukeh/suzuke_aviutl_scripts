# プロジェクト概要

## 基本情報

- **プロジェクト名**: suzuke_aviutl_scripts
- **作者**: Suzuke
- **リポジトリ**: `/mnt/c/AviUtl2/data/Script/Suzuke`
- **ライセンス**: MIT（二次創作・改変はcopyleft）
- **設置先**: `AviUtl/script/すずけ/` に全スクリプトを配置

## プロジェクトの性質

AviUtl / AviUtl2 向けの多種多様なLuaスクリプトの詰め合わせ。安定版と実験的スクリプトが混在している。READMEでも「ゴミが散らばっている」と自認している状態。

## 対応プラットフォーム

| プラットフォーム | 拡張子 | エンコーディング |
|------------------|--------|-----------------|
| AviUtl (ExEdit) | `.anm`, `.obj`, `.tra` | Shift_JIS |
| AviUtl2 (ExEdit2) | `.anm2`, `.obj2` | UTF-8 |

## Lua仕様

- **AviUtl**: Lua 5.1 ベース
- **AviUtl2**: LuaJIT ベース（高速化）、HLSLシェーダサポート

## 開発方針

1. AviUtl2 スクリプト（`.anm2`, `.obj2`）を中心に開発
2. 既存の AviUtl スクリプトの AviUtl2 移植も検討
3. 各スクリプトは1ファイル完結（グローバル汚染禁止、local変数使用）
4. ユーザー向けパラメータ名は日本語
5. 構文チェックは `luac -p` または AviUtl2 実機で確認

## 参照すべき技術資料

- `AviUtl2_Script_Guide.md` — AviUtl2 API リファレンス
- `解説/` ディレクトリ — 各スクリプトの解説ドキュメント
- `Sample/` — AviUtl2 サンプルプロジェクト
