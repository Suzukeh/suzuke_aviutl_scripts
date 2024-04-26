# suzuke_aviutl_scripts
すずけのスクリプトを雑多に置く。

作り途中でGitHubを使ったためゴミが散らばっている。

以下はまだゴミ。
> * @個別.anm
> * @並べる.anm

## 導入方法
`AviUtl/script/すずけ` などscriptフォルダ内の適当な場所にスクリプトと関連ファイルを置く。

わからなければ https://scrapbox.io/aviutl/スクリプトの導入方法 を参照。

依存関係は次の通り。

|スクリプト|関連ファイル|
|---|---|
|@VLL_Suzuke.obj|char_height_list.csv|


## 各ファイルの概要
### @VLL_Suzuke.obj
所属するサークル(VLL)の関連で使うことを想定しているカスタムオブジェクト。
#### 身長補助線
キャラの身長の目安になる補助線を引く。Unity用のやつはあったけど、AviUtlでも使いたくなったので作った。

身長がはっきりしてるボカロキャラクターはプリセットに登録してある。

[解説/身長補助線](https://github.com/Suzukeh/suzuke_aviutl_scripts/tree/main/%E8%A7%A3%E8%AA%AC/%E8%BA%AB%E9%95%B7%E8%A3%9C%E5%8A%A9%E7%B7%9A)
