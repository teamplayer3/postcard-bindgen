use derive_enum::derive_enum;
use derive_struct::derive_struct;
use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{ast, Ctxt, Derive};
use syn::DeriveInput;

mod derive_enum;
mod derive_struct;

const PRIVATE_IMPORT_PATH: &str = "_pb::__private";

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
    let container_name = ident.to_string();

    let expanded = if cfg!(feature = "expanding") {
        quote!(
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate postcard_bindgen as _pb;
                impl #PRIVATE_IMPORT_PATH::JsBindings for #ident {
                    fn create_bindings(reg: &mut #PRIVATE_IMPORT_PATH::BindingsRegistry) {
                        #body
                    }
                }

                impl #PRIVATE_IMPORT_PATH::GenJsBinding for #ident {
                    fn get_type() -> #PRIVATE_IMPORT_PATH::JsType {
                        #PRIVATE_IMPORT_PATH::JsType::Object(#PRIVATE_IMPORT_PATH::ObjectMeta {
                            name: #container_name.into()
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
