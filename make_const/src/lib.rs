use proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

#[cfg(feature = "enabled")]
#[proc_macro_attribute]
pub fn const_if_feature_enabled(_: TokenStream, item: TokenStream) -> TokenStream {
    let string = item.to_string();
    let fn_index = string.find("fn").unwrap();
    let res = format!("{}{} {}", &string[0..fn_index], "const", &string[fn_index..]);
    TokenStream::from_str(&res).unwrap()
}

#[cfg(not(feature = "enabled"))]
#[proc_macro_attribute]
pub fn const_if_feature_enabled(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}