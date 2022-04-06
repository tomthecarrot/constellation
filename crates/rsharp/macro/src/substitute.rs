use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{self, visit_mut::VisitMut, Error, Result};

struct VisitAndSub {
    value: TokenStream,
}
impl VisitAndSub {
    pub fn new(value: TokenStream) -> Result<Self> {
        syn::parse2::<syn::Expr>(value.clone())?;
        Ok(Self { value })
    }
}
impl VisitMut for VisitAndSub {
    fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
        use syn::Expr as E;
        let new_node = match node {
            E::Macro(ref mut m) => {
                let macro_name = m.mac.path.segments.last().unwrap().ident.to_string();
                if macro_name == "substitute" {
                    if !m.attrs.is_empty() {
                        return; // Don't try to step on another substitution's toes
                    }
                    Some(syn::parse2::<syn::Expr>(self.value.clone()).unwrap())
                } else {
                    None
                }
            }
            _ => return syn::visit_mut::visit_expr_mut(self, node),
        };
        if let Some(new_node) = new_node {
            *node = new_node;
        }
        syn::visit_mut::visit_expr_mut(self, node);
    }

    fn visit_attribute_mut(&mut self, node: &mut syn::Attribute) {
        let token_str = node.tokens.to_string();

        let mut try_pattern = |pattern| {
            let splits: Vec<_> = token_str.split(pattern).into_iter().collect();
            if splits.len() > 1 {
                let new_str = splits.as_slice().join(&self.value.to_string());
                let new_tokens = TokenStream::from_str(&new_str)
                    .expect("Didn't expect to be unable to parse tokenstream from attr");
                node.tokens = new_tokens;
                Ok(())
            } else {
                Err(())
            }
        };

        let _result = try_pattern("substitute! ()").or_else(|_| try_pattern("substitute ! ()"));

        syn::visit_mut::visit_attribute_mut(self, node);
    }
}

pub fn substitute(attr: TokenStream, mut item: syn::Item) -> Result<TokenStream> {
    if attr.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "Must provide the tokens to substitute",
        ));
    }

    let mut visitor = VisitAndSub::new(attr)?;
    syn::visit_mut::visit_item_mut(&mut visitor, &mut item);
    Ok(item.to_token_stream())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_substitute() -> Result<()> {
        let examples: Vec<(TokenStream, syn::Item, TokenStream)> = vec![
            (
                quote! {"hello"},
                syn::parse_quote! {
                    mod my {
                        pub fn asdf() {
                            let a = substitute!();
                            let b = {
                                substitute!()
                            };
                        }
                    }
                },
                quote! {
                    mod my {
                        pub fn asdf() {
                            let a = "hello";
                            let b = {
                                "hello"
                            };
                        }
                    }
                },
            ),
            (
                quote! { Default },
                syn::parse_quote! {
                    #[derive(substitute!())]
                    struct MyStruct;
                },
                quote! {
                    #[derive(Default)]
                    struct MyStruct;
                },
            ),
            (
                quote! { Default },
                syn::parse_quote! {
                    #[derive(substitute !())]
                    struct MyStruct;
                },
                quote! {
                    #[derive(Default)]
                    struct MyStruct;
                },
            ),
            (
                quote! { Default },
                syn::parse_quote! {
                    #[derive(substitute ! ())]
                    struct MyStruct;
                },
                quote! {
                    #[derive(Default)]
                    struct MyStruct;
                },
            ),
        ];
        for (input, item, expected) in examples {
            let tokens = substitute(input, item)?;
            assert_eq!(tokens.to_string(), expected.to_string());
        }

        Ok(())
    }
}
