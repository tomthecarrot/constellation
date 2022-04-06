use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{self, Error, Result};

fn mangle_path(m: &str) -> Result<String> {
    let mut result = String::with_capacity(m.len());
    for module in m.split("::") {
        if module.is_empty() {
            continue;
        }
        result.push_str(module.trim());
        result.push_str("__");
    }
    Ok(result)
}

pub fn remangle(path: syn::LitStr, mut item: syn::Item) -> Result<TokenStream> {
    let path = path.value();
    let mangle = |ident: &mut syn::Ident| -> Result<()> {
        *ident = syn::Ident::new(
            &format!("{}{}", &mangle_path(&path)?, ident.to_string()),
            ident.span(),
        );
        Ok(())
    };
    use syn::Item::*;
    match item {
        Struct(ref mut i) => mangle(&mut i.ident),
        Fn(ref mut f) => mangle(&mut f.sig.ident),
        Enum(ref mut e) => mangle(&mut e.ident),
        _ => return Err(Error::new(item.span(), "This item type is not supported")),
    }?;

    Ok(item.into_token_stream())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mangle_path() -> Result<()> {
        let inputs = [
            ("::crate::a::b::c", "crate__a__b__c__"),
            ("crate::a::b::c", "crate__a__b__c__"),
            ("crate", "crate__"),
            ("::", ""),
        ];
        for (path, mangled_path) in inputs {
            assert_eq!(mangle_path(path)?, mangled_path);
        }
        Ok(())
    }
}
