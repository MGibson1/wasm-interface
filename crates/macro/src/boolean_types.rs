use quote::quote;

pub fn bool_types() -> [String;2] {
    [
        quote! { bool }.to_string(),
        quote! { Option<bool> }.to_string()
    ]
}
