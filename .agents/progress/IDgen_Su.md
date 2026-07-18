# IDgen_Su

## 状況
✅ 完了 (v1.0.0 リリース済み)

## リポジトリ
https://github.com/Suzukeh/IDgen_Su

## 設置先
`Script/Suzuke.IDgen_Su/`

## 対応内容
- 8種のID生成カスタムオブジェクト (UUIDv4, NanoID, ULID, CUID, UUIDv7, ShortID, KSUID, Snowflake)
- `obj.rand1()` ベースの乱数生成、毎フレーム更新/オブジェクト毎変化のチェックボックス
- ドロップダウンによるID種別選択 (`--select@`)
- ID生成ロジックを `IDgen_Su.lua` に分離（他スクリプトからも再利用可能）
- au2pkg パッケージ作成 (package.ini + package.txt CRLF + zip)
- GitHub Release v1.0.0
- AviUtl2 カタログ登録済み
- README にカタログバッジを設置
- リリース手順を `.agents/knowledge/release-procedure.md` に文書化

## ファイル構成
```
@IDgen_Su.obj2    # エントリポイント (UI, 状態管理, 描画)
IDgen_Su.lua      # ID生成モジュール (各ジェネレータ)
README.md
build/
  ├── Suzuke.IDgen_Su.au2pkg.zip
  └── pkg/
```
