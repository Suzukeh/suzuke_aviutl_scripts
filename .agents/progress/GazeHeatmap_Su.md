# GazeHeatmap_Su

## 状況
計画中

## 対応内容
- 設計方針策定:
  - CSV / インジェクション / マウス録画 の3モード対応
  - 蓄積バッファ + ガウシアンスプラット + カラーマップ のパイプライン
  - HLSLシェーダー内蔵 (`ps_gaussian`, `ps_colormap`)
  - `PBuffer_Su` 相当のフレーム間状態永続化
  - パラメーターインジェクション (`--value@PI`) 対応

## TODO
- [ ] `@GazeHeatmap_Su.anm2` 実装
  - [ ] CSVパーサー
  - [ ] ps_gaussian （ガウス分布スプラット）
  - [ ] ps_colormap （カラーマップ + 合成）
  - [ ] 蓄積バッファ管理
  - [ ] マウス録画モード
- [ ] AviUtl2 上で実機検証
