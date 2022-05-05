use wasm_interfacegen_macro::JsInterface;


#[derive(JsInterface)]
enum OnAnEnum{}

#[derive(JsInterface)]
union OnAUnion {
    field: i8,
}

pub fn main(){}
