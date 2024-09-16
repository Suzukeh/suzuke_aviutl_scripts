# suzuke_aviutl_scripts

すずけのスクリプトを雑多に置いています。

作り途中でGitHubを使ったためゴミが散らばっています。

以下のもの以外はまだゴミ。
> 身長補助線@VLL_Suzuke.obj
> 斜めブラインド_Su.anm
> Hide.anm

ライセンスは、他に原著作者の存在するものはそれのコピーレフト、そうでないものはMITライセンスとしておきます。
<details>
  Copyright 2024 Suzuke

  Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.</details>

## 導入方法

`AviUtl/script/すずけ` などscriptフォルダ内の適当な場所にスクリプトと関連ファイルを置く。

|スクリプト|関連ファイル|
|---|---|
|[@VLL_Suzuke.obj](https://github.com/Suzukeh/suzuke_aviutl_scripts/blob/main/%40VLL_Suzuke.obj)|[char_height_list.csv](https://github.com/Suzukeh/suzuke_aviutl_scripts/blob/main/char_height_list.csv)|

## 各ファイルの概要

### @VLL_Suzuke.obj

所属するサークル(VLL)の関連で使うことを想定しているカスタムオブジェクト。

#### 身長補助線

キャラの身長の目安になる補助線を引く。Unity用のやつはあったけど、AviUtlでも使いたくなったので作った。

身長がはっきりしてるボカロキャラクターはプリセットに登録してあります。

[解説/身長補助線](https://github.com/Suzukeh/suzuke_aviutl_scripts/tree/main/%E8%A7%A3%E8%AA%AC/%E8%BA%AB%E9%95%B7%E8%A3%9C%E5%8A%A9%E7%B7%9A)

### 斜めブラインド_Su.anm

timさんの[斜めブラインド(改)](https://www.nicovideo.jp/watch/sm17155254)を改変して基準の範囲を大きくしました。<br>~~配布するまでもないとは思う~~

使い方は変えてないので元のスクリプトの解説を見てください。

いくらでも大きく設定できるように処理を書き換えたので、各自必要なだけスクリプトの `--track3:基準,-1000,1000` の部分を大きくするとよいかと。

### Hide.anm

レンダリング時に非表示にするアニメーション効果です。
一応プレビュー時に非表示にするとか、常時非表示とかも設定できるようにしてあります。
