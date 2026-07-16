# 進捗メモ

> 最終更新: 2026-07-15

## 現在の状況

- [x] IDgen_Su スクリプト作成・仕様準拠確認
- [x] au2pkg パッケージ作成
- [x] GitHub Release (v1.0.0) 発行
- [x] 独立リポジトリ `Suzukeh/IDgen_Su` に移行
- [x] リリース手順を文書化 (`.agents/knowledge/release-procedure.md`)

## 次にやること

## 詰まっていること

（特になし）

---

## 過去の完了タスク

- IDgen_Su 開発: 8種のID生成カスタムオブジェクト
  - UUIDv4, NanoID, ULID, CUID, UUIDv7, ShortID, KSUID, Snowflake
  - `obj.rand1()` ベースの乱数生成、毎フレーム更新/オブジェクト毎変化のチェックボックス
  - ドロップダウンによるID種別選択 (`--select@`)
  - au2pkg パッケージ作成 (package.ini + package.txt CRLF + zip)
  - GitHub Release v1.0.0
  - 独立リポジトリ: https://github.com/Suzukeh/IDgen_Su
  - 設置先: `Script/Suzuke.IDgen_Su/`
- `.agents/` 知識ベース整備
- AIコーディングエージェントのベストプラクティス調査・文書化
- AviUtl2 スクリプト作成ガイド文書化
