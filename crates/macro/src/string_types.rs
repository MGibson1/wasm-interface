use quote::quote;

pub fn string_types() -> [String; 4] {
    [
        quote! { String }.to_string(),
        quote! { Option<String> }.to_string(),
        quote! { &str }.to_string(),
        quote! { char }.to_string()
    ]
}
