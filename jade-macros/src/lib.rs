extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    for token in _item {
        println!("{:?}", token);
    }
    "fn answer() -> u32 { 42 }".parse().unwrap()
}