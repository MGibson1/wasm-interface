
// Trait used to indicate that a struct has been decorated with
// the #[derive(JsInterface)] procedural macro
pub trait __JsInterface {
    fn js_interface_name() -> &'static str;
}
