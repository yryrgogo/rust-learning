use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is pancakes!");
//     }
// }
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is pancakes!");
//     }
// }

fn main() {
    Pancakes::hello_macro();
}
