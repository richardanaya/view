extern crate proc_macro;

use proc_macro::{Group, TokenStream, TokenTree};
use std::error::Error;
use std::iter::Peekable;
use std::str::FromStr;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    TokenStream::from_str(r#"Button{text:"blah".to_owned()}"#).unwrap()
}

#[proc_macro_derive(View)]
pub fn my_macro_here_derive(input: TokenStream) -> TokenStream {
    println!("{:?}",input); 
    TokenStream::from_str(r#"impl Foo for Button {}"#).unwrap()
}