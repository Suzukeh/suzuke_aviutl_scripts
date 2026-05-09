--[[
1のキャッシュバッファが永続(Persistent)化されていた性質を利用しているスクリプトを2に移植するためのモジュール的なもの

2ではキャッシュバッファがフレームごとに消えてしまう。そこで、本モジュールでは2でも永続なグローバル変数にキャッシュバッファの内容を保存する。

・使い方の例
まずはスクリプトの先頭あたりでrequreしておく
local pb = require("PBuffer_Su")

1. 退避
従来
obj.copybuffer("cache:xxx", "obj") --キャッシュバッファに退避

移植
obj.copybuffer("cache:xxx", "obj") --これだとフレームごとにキャッシュが消えてしまうので
pb_globalval = pb.set("cache:xxx") --これを使ってグローバル変数に保存

2. 復元
従来
obj.copybuffer("tempbuffer", "cache:xxx") --キャッシュバッファからtmpとか復元

移植

obj.putpixeldata("tempbuffer", data, w, h, "rgba")

]]

--バッファ用グローバル変数
-- { {buffername, w, h,pixeldata} , ... }
PBuffer_Su = PBuffer_Su or {}

PBuffer_Su.setfromcache = function(drawtarget, w, h)

end


return PBuffer_Su
