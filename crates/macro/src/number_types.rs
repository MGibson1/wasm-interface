use quote::quote;

pub fn number_types() -> [String; 24] {
    [
        quote! { u8 }.to_string(),
        quote! { i8 }.to_string(),
        quote! { u16 }.to_string(),
        quote! { i16 }.to_string(),
        quote! { u32 }.to_string(),
        quote! { i32 }.to_string(),
        quote! { u64 }.to_string(),
        quote! { i64 }.to_string(),
        quote! { usize }.to_string(),
        quote! { isize }.to_string(),
        quote! { f32 }.to_string(),
        quote! { f64 }.to_string(),
        quote! { Option<u8> }.to_string(),
        quote! { Option<i8> }.to_string(),
        quote! { Option<u16> }.to_string(),
        quote! { Option<i16> }.to_string(),
        quote! { Option<u32> }.to_string(),
        quote! { Option<i32> }.to_string(),
        quote! { Option<u64> }.to_string(),
        quote! { Option<i64> }.to_string(),
        quote! { Option<usize> }.to_string(),
        quote! { Option<isize> }.to_string(),
        quote! { Option<f32> }.to_string(),
        quote! { Option<f64> }.to_string(),
    ]
}
