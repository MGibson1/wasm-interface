use quote::quote;

pub fn array_types() -> [String;2] {
    [
        quote! { Box<[wasm_bindgen::prelude::JsValue]> }.to_string(),
        quote! { Option<Box<[wasm_bindgen::prelude::JsValue]>> }.to_string()
    ]
}
