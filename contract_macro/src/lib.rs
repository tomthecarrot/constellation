use proc_macro2::TokenStream;
use quote::quote_spanned;
use quote::ToTokens;
use syn::parse2;
use syn::parse_macro_input;
use syn::spanned::Spanned;

// channel and state proc macros are really similar, so to avoid boilerplate and
// keep it DRY we provide a template macro.
macro_rules! template {
    ($macro_name:ident, $handle_type:ty) => {
        #[proc_macro_attribute]
        pub fn $macro_name(
            _attr: proc_macro::TokenStream,
            item: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            // Get the AST of anything that can be derived (in this case, the struct)
            let mut item = parse_macro_input!(item as syn::DeriveInput);

            let s = if let syn::Data::Struct(ref mut s) = item.data {
                s
            } else {
                // Can become a span error when `proc_macro_diagnostics` is stabilized
                panic!("Only structs are supported")
            };

            // Represents contents of impl blocks
            let mut impl_ts = TokenStream::new();
            let s_name = &item.ident;

            let fields = if let syn::Fields::Named(ref mut fields) = s.fields {
                fields.named.iter_mut()
            } else {
                panic!("Only named structs are supported")
            };

            for f in fields {
                let inner_t = f.ty.clone();
                let f_name = f.ident.as_ref().expect("Fields should be named");

                f.ty = parse2(quote_spanned! {inner_t.span()=>
                    $handle_type<#inner_t>
                })
                .unwrap();
                let wrapped_ty = &f.ty;

                impl_ts.extend(quote_spanned! {inner_t.span()=>
                    impl #s_name {
                        pub fn #f_name(&self) -> #wrapped_ty {
                            self.#f_name
                        }
                    }
                });
            }

            // Concatenate and retunrn item tokens and impl tokens
            let mut result = item.into_token_stream();
            result.extend(impl_ts);
            result.into()
        }
    };
}

template!(states, tp_client::contract::properties::StateId);
template!(channels, tp_client::contract::properties::ChannelId);
