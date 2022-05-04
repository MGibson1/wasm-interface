use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, spanned::Spanned};
use quote::quote;

mod boolean_types;
mod string_types;
mod number_types;

#[proc_macro_derive(JsInterface)]
pub fn wasm_interface(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_ident = &input.ident;
    let iname = format!("I{}", input.ident);
    let iident = syn::Ident::new(&iname, input.ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed {ref named, .. }),
        ..
    }) = input.data {
        named
    } else {
        unimplemented!();
    };

    let js_fields = map_r_fields_to_js_fields(fields);

    if let Err(e) = js_fields {
        return e.to_compile_error().into();
    }

    let mut idef = format!("interface {} {{ ", iname);
    idef.push_str(&js_fields.unwrap().join(" "));
    idef.push_str(" }");
    
    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(typescript_custom_section)]
        const ITEXT_STYLE: &'static str = #idef;
        
        #[wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {
            #[wasm_bindgen::prelude::wasm_bindgen(typescript_type= #iname)]
            pub type #iident;
        }

        impl wasm_interface::__JsInterface for #input_ident {
            fn js_interface_name() -> &'static str {
                #iname
            }
        }
    }.into()
}

fn map_r_fields_to_js_fields(fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>) -> Result<Vec<String>, syn::Error> {
    let mut result: Vec<String> = Vec::new();
    for f in fields.iter() {
        let name = &ty_name(&f.ty);
        let js_type: &str;

        if string_types::string_types().iter().any(|t| t == name) {
            js_type = "string";
        } else if number_types::number_types().iter().any(|t| t == name) {
            js_type = "number";
        } else if boolean_types::bool_types().iter().any(|t| t == name) {
            js_type = "boolean";
        } else {
            return Err(syn::Error::new(f.ty.span(), format!("Unknown type, {}; only string, number, and boolean js types are currently supported", name)));
        }
        result.push(format!("{}: {};", f.ident.as_ref().unwrap(), js_type));
    }

    Ok(result)
}

fn ty_name(ty: &syn::Type) -> String {
    quote!(#ty).to_string()
}

// Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "String", span: #0 bytes(141..147) }, arguments: None }] } })

// fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
//     if let syn::Type::Path(ref p) = ty {
//         if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
//             return None;
//         }

//         if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
//             if inner_ty.args.len() != 1 {
//                 return None;
//             }

//             let inner_ty = inner_ty.args.first().unwrap();
//             if let syn::GenericArgument::Type(ref t) = inner_ty.value() {
//                 return Some(t);
//             }
//         }
//     }
//     None
// }
