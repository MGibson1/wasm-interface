use wasm_interfacegen_macro::JsInterface;

pub fn main(){}

#[derive(JsInterface)]
struct VecField {
    vec_field: Vec<String>
}

#[derive(JsInterface)]
struct NonInterfacedField {
    non_interfaced_field: SomeOtherStruct
}

struct SomeOtherStruct{}
