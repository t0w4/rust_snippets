# rust_snippets

## おすすめ拡張機能(vs code)

```json
{
    "rust-analyzer.checkOnSave.command": "clippy", // checkにclippyを使用
    "editor.formatOnSave": true, // 保存時に cargo fmtを実行 
}
```

## おすすめ拡張機能(cargo)

### cargo-edit

以下のコマンドを使えるようになる()

- cargo add: `cargo add {ライブラリ名}` でCargo.tomlのdependenciesに追加してくれる
- cargo rm: `cargo rm {ライブラリ名}` でCargo.tomlのdependenciesから削除してくれる
- cargo upgrade
- cargo set-version

```shell
cargo install cargo-edit
```

install時にエラーが出る場合は`rustup update`でバージョンを上げると解消する場合もある。

成功時のバージョン

```shell
$ rustc --version   
rustc 1.60.0 (7737e0b5c 2022-04-04)
$ cargo --version    
cargo 1.60.0 (d1fd9fe2c 2022-03-01)
```
