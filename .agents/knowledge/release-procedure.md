# リリース手順 (IDgen_Su)

## au2pkg の作成

```bash
# 1. pkg ディレクトリ構成を整える
#    build/pkg/
#    ├── package.ini
#    ├── package.txt          # 改行コードは CRLF (unix2dos または sed -i 's/$/\r/')
#    └── Script/Suzuke.IDgen_Su/
#        ├── @IDgen_Su.obj2
#        └── README.md

# 2. zip 化 (Python: ファイルは DEFLATE、ディレクトリは STORE)
cd build/pkg
python3 -c "
import zipfile, os
with zipfile.ZipFile('../Suzuke.IDgen_Su.au2pkg.zip', 'w', zipfile.ZIP_DEFLATED) as z:
    for root, dirs, files in os.walk('.'):
        for f in files:
            path = os.path.join(root, f)
            arcname = path.lstrip('./') if path.startswith('./') else path
            z.write(path, arcname)
"

# 3. 検証
unzip -t ../Suzuke.IDgen_Su.au2pkg.zip
```

## package.ini
```ini
[package]
id=Suzuke.IDgen_Su
name=IDgen_Su
information=UUID@IDgen_Su v1.0.0 by Suzuke
```

## package.txt
- 内容: インストール時のダイアログに表示されるテキスト
- 改行コード: **CRLF 必須**（LFだとAviUtl2で改行が反映されない）

## GitHub Release
```bash
# アセット更新
gh release upload v1.0.0 build/Suzuke.IDgen_Su.au2pkg.zip

# 既存アセット差し替え
gh release delete-asset v1.0.0 Suzuke.IDgen_Su.au2pkg.zip -y
gh release upload v1.0.0 build/Suzuke.IDgen_Su.au2pkg.zip
```

## 注意点
- au2pkg 内のファイルパスは `./` プレフィックスを付けない（AviUtl2側でエラーになる可能性あり）
- zip 内のディレクトリエントリは圧縮しない（STORE、compress_type=0）
- ファイルは DEFLATE（compress_type=8）で問題ない
- パッケージ識別子 (`id`) が同じ場合、再インストール時に旧ファイルが自動アンインストールされる
- `Script/` 以下のみが実際にインストールされる。`package.ini` や `package.txt` はルートに置く（インストール対象外）
