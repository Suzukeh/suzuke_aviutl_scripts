# AviUtl2 スクリプティング原則

## obj.draw() を使わない

アニメーション効果では、描画結果を `obj.draw()` でフレームバッファに直接描画してはいけない。
`obj.draw()` を使うと後続のエフェクトが効かなくなる。

**正しいパターン:**
```lua
-- tempbuffer に描画
obj.setoption("drawtarget", "tempbuffer", w, h)
obj.clearbuffer("tempbuffer")
-- ... 描画処理 ...
-- object バッファにコピーするだけ（drawしない）
obj.copybuffer("object", "tempbuffer")
```

この原則は `@入退場.anm2` のように `obj.alpha` 等のプロパティを変更するだけの
エフェクトを除き、すべての描画系アニメーション効果に適用する。
