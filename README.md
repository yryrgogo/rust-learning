# rust-starting

Rust Starting

## basic command

### create project

`cargo new <project_name>`

### compile/build

`cargo build`

#### compile/build & run

`cargo run`

### cleaner

`cargo clean`

## Memo

- 所有権というのは「固定メモリ」を示すことができる権限
- 代入によって所有権は「移動」する。関数の引数に指定した場合も「移動」する。
- 変数が移動によって空になった場合、`println!` のマクロは変数から「借用」ができない

```rust
fn main() {
    let x= String::from("Hello");
    let y = x ; // 所有権の移動が発生
    println!("x is {}", x); // x は String::from("Hello") が格納された固定メモリの所有権を失っているのでエラーになる。println!マクロが変数 x を借りて値を表示しようとするが失敗する
    println!("y is {}", y); // y は x から値の所有権が移動されたので println! マクロが値を表示（借用）できる
}
```

- 関数の引数では「参照」を使えば「移動」は起こらない

## Question

- 文字列の型の `&str` にある `&` とは？ -> `&` は「参照」の印
- 型強制とは？ -> 思ったよりむずいので時間とって勉強する(https://doc.rust-jp.rs/rust-nomicon-ja/coercions.html)
