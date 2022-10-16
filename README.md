# rust_tutorial
This repo is for rust tutorial.
## Set Up
### install
```shell
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
...
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1
...
Rust is installed now. Great!
...
source "$HOME/.cargo/env"
```

### version check
```shell
cargo --version
cargo 1.64.0 (387270bc7 2022-09-16)
```


## Memo
* debug setting for vscode
    * https://stackoverflow.com/questions/70723735/cannot-debug-rust-in-visual-studio-code

## Reference
* doc
    * https://www.rust-lang.org/
* how to install
    * https://doc.rust-lang.org/book/ch01-01-installation.html
* Library
    * rayon
        * https://github.com/rayon-rs/rayon
    * structopt
        * https://github.com/TeXitoi/structopt
* Zenn
    * https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/6d5875


## Nest Step
* Create Command Line Tool
    * file converter
* https://qiita.com/watawuwu/items/a6cbcd92dfb5336b9a01
* https://speakerdeck.com/helloyuk13/rusthanzuon-at-eurekashe