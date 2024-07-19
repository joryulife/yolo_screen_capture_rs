# about this

## 環境構築

1. Rustのインストール
Rustがインストールされていない場合、以下のコマンドでインストールします。

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. YOLOv7の準備
YOLOv7の事前学習済みモデルをダウンロードし、プロジェクトディレクトリに保存します。
```sh
New-Item -ItemType Directory -Path "weights" 
Invoke-WebRequest -Uri "https://github.com/WongKinYiu/yolov7/releases/download/v0.1/yolov7.pt" -OutFile "weights/yolov7.pt"
```

3. libtorchのダウンロード
(公式)[https://pytorch.org/]からC++/javaのものをダウンロード

4. 環境変数の設定
ダウンロードしたlibtorchのパスを環境変数に設定します。

libtorchを展開し、フォルダのパス（例: C:\path\to\libtorch）をコピーします。
環境変数LIBTORCHを設定します。Windowsの設定から、システム環境変数を開き、新しいシステム変数を追加します。

```sh
LIBTORCH=C:\path\to\libtorch
```

また、PATHに以下のパスを追加します。

```sh
C:\path\to\libtorch\bin
```

5. torch-sysのビルド設定の確認
torch-sysクレートがlibtorchの場所を認識できるように、build.rsファイルをプロジェクトに追加し、libtorchのパスを設定します。

- build.rsの作成
プロジェクトのルートディレクトリにbuild.rsを作成し、以下の内容を追加します。

```rust
fn main() {
    // Libtorchのインクルードとライブラリパスを設定
    println!("cargo:rustc-link-search=native={}/lib", std::env::var("LIBTORCH").unwrap());
    println!("cargo:rustc-link-lib=static=c10");
    println!("cargo:rustc-link-lib=static=torch_cpu");
    println!("cargo:rustc-link-lib=static=torch");
}
```

6. ビルド

```sh
cargo clean
cargo build --release
```


