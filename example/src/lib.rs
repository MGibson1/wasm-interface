use wasm_interfacegen::*;

#[derive(JsInterface)]
pub struct Request {
    num: f32,
    an_option: Option<bool>,
    data: SubRequest
}

#[derive(JsInterface)]
pub struct SubRequest {
    data: String,
    an_option: Option<i32>
}
