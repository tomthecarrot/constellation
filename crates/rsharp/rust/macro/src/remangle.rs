use proc_macro2::TokenStream;
use quote::quote_spanned;
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
    let mangle = |ident: &mut syn::Ident| -> Result<(syn::Ident, syn::Ident)> {
        let original_ident = ident.clone();
        *ident = syn::Ident::new(
            &format!("{}{}", &mangle_path(&path)?, ident.to_string()),
            ident.span(),
        );
        Ok((original_ident, ident.clone()))
    };
    use syn::Item::*;
    let (original_ident, mangled_ident) = match item {
        Struct(ref mut i) => mangle(&mut i.ident),
        Fn(ref mut f) => mangle(&mut f.sig.ident),
        Enum(ref mut e) => mangle(&mut e.ident),
        _ => return Err(Error::new(item.span(), "This item type is not supported")),
    }?;

    let output = quote_spanned! {item.span() =>
        #item
        #[allow(unused)]
        use #mangled_ident as #original_ident;
    };

    Ok(output)
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
