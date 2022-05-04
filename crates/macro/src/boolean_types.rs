use quote::quote;

pub fn bool_types() -> [String;1] {
    [quote! { bool }.to_string()]
}
