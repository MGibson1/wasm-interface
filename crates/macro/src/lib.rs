use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

mod boolean_types;
mod string_types;
mod number_types;
mod any_types;
mod array_types;

#[proc_macro_derive(JsInterface)]
pub fn wasm_interfacegen(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_ident = &input.ident;
    let iname = format!("I{}", input.ident);
    let iident = syn::Ident::new(&iname, input.ident.span());

    if let syn::Data::Enum(syn::DataEnum {ref enum_token, ..}) = input.data {
        return syn::Error::new(enum_token.span, "JsInterface only supports structs").to_compile_error().into();
    } else if let syn::Data::Union(syn::DataUnion {ref union_token, ..}) = input.data {
        return syn::Error::new(union_token.span, "JsInterface only supports structs").to_compile_error().into();
    }

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed {ref named, .. }),
        ..
    }) = input.data {
        named
    } else {
        unimplemented!();
    };

    let idef_ident = syn::Ident::new(&format!("{}_STYLE", iname.clone().to_uppercase()), input.ident.span());
    let mut idef = format!("interface {} {{ ", iname);
    let mut req_impls: Vec<&syn::Type> = Vec::new();
    for field in fields.iter() {
        let js_type = match map_r_type_to_js_type(&field.ty) {
            Ok(t) => t,
            Err(e) => return e.to_compile_error().into()
        };
        
        let field_ident = field.ident.as_ref().unwrap();
        if !is_simple_type(&js_type) {
            req_impls.push(&field.ty);
        }
        idef.push_str(&format!("{}: {}; ", field_ident, js_type))
    }
    idef.push_str("}");
    
    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(typescript_custom_section)]
        const #idef_ident: &'static str = #idef;

        #(wasm_interfacegen::assert_impl_all!(#req_impls: wasm_interfacegen::__JsInterface);)*
        
        #[wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {
            #[wasm_bindgen::prelude::wasm_bindgen(typescript_type= #iname)]
            pub type #iident;
        }

        impl wasm_interfacegen::__JsInterface for #input_ident {
            fn js_interface_name() -> &'static str {
                #iname
            }
        }
    }.into()
}

fn map_r_type_to_js_type(ty: &syn::Type) -> Result<String, syn::Error> {
    let name = &ty_name(ty);
    let js_type: String;

    if string_types::string_types().iter().any(|t| t == name) {
        js_type = "string".into();
    } else if number_types::number_types().iter().any(|t| t == name) {
        js_type = "number".into();
    } else if boolean_types::bool_types().iter().any(|t| t == name) {
        js_type = "boolean".into();
    } else if any_types::any_types().iter().any(|t| t == name) {
        js_type = "any".into();
    } else if array_types::array_types().iter().any(|t| t == name) {
        js_type = "array".into();
    } else {
        js_type = format!("I{}", name);
    }

    Ok(js_type)
}

fn is_simple_type(js_type: &str) -> bool {
    if js_type == "string" || js_type == "number" || js_type == "boolean" || js_type == "any" {
        return true;
    }
    false
}

fn ty_name(ty: &syn::Type) -> String {
    quote!(#ty).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn try_compile_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/*.rs");
    }
}
