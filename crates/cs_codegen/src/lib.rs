pub struct Class {
    fns: Vec<FnDesc>,
    name: syn::Ident,
}
impl Class {
    // pub fn generate(class_name: &str)
}

pub struct FnDesc {
    item: syn::ItemFn,
}
