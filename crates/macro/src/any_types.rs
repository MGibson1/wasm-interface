use quote::quote;

pub fn any_types() -> [String; 4] {
    [
        quote! { wasm_bindgen::prelude::JsValue }.to_string(),
        quote! { &wasm_bindgen::prelude::JsValue }.to_string(),
        quote! { Option<wasm_bindgen::prelude::JsValue> }.to_string(),
        quote! { Option<&wasm_bindgen::prelude::JsValue> }.to_string(),
    ]
}
