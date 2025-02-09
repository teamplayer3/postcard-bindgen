use derive_enum::derive_enum;
use derive_struct::derive_struct;
use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{ast, Ctxt, Derive};
use syn::DeriveInput;

mod derive_enum;
mod derive_struct;

#[proc_macro_derive(PostcardBindings)]
pub fn postcard_bindings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_js_implementation(input).into()
}

fn derive_js_implementation(input: proc_macro::TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let cx = Ctxt::new();
    let container = ast::Container::from_ast(&cx, &input, Derive::Serialize).unwrap();

    let body = match container.data {
        ast::Data::Enum(variants) => derive_enum(container.ident.to_owned(), variants),
        ast::Data::Struct(style, fields) => {
            derive_struct(style, container.ident.to_owned(), fields)
        }
    };

    let ident = container.ident;
    let generics = container.generics;
    let container_name = ident.to_string();

    let expanded = if cfg!(feature = "expanding") {
        quote!(
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate postcard_bindgen as _pb;
                impl #generics _pb::__private::JsBindings for #ident #generics {
                    fn create_bindings(reg: &mut _pb::__private::BindingsRegistry) {
                        #body
                    }
                }

                impl #generics _pb::__private::GenBinding for #ident #generics {
                    fn get_type() -> _pb::__private::ValueType {
                        _pb::__private::ValueType::Object(_pb::__private::ObjectMeta {
                            name: #container_name.into(),
                            path: _pb::__private::Path::new(module_path!(), "::"),
                        })
                    }
                }
            };
        )
    } else {
        TokenStream::new()
    };

    cx.check().unwrap();

    expanded
}
