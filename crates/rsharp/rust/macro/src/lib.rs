mod remangle;
mod substitute;

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn remangle(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    let literal = parse_macro_input!(attr as syn::LitStr);
    remangle::remangle(literal, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn substitute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as syn::Item);
    substitute::substitute(attr.into(), item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
