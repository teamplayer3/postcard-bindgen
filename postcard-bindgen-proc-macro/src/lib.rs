use derive_struct::derive_struct;
use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{ast, Ctxt, Derive};
use syn::DeriveInput;

mod derive_struct;

#[proc_macro_derive(PostcardBindings)]
pub fn postcard_bindings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_js_implementation(input).into()
}

fn derive_js_implementation(input: proc_macro::TokenStream) -> TokenStream {
    // eprintln!(".........[input] {}", input);
    let input: DeriveInput = syn::parse(input).unwrap();

    let cx = Ctxt::new();
    let container = ast::Container::from_ast(&cx, &input, Derive::Serialize).unwrap();

    let body = match container.data {
        ast::Data::Enum(_) => unimplemented!(),
        ast::Data::Struct(style, fields) => {
            derive_struct(style, container.ident.to_owned(), fields)
        }
    };

    let ident = container.ident;
    let container_name = ident.to_string();

    let expanded = if cfg!(any(debug_assertions, feature = "export-js")) {
        quote!(
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate postcard_bindgen as _pb;
                impl _pb::JsBindings for #ident {
                    fn create_bindings(reg: &mut _pb::BindingsRegistry) {
                        #body
                    }
                }

                impl _pb::GenJsBinding for #ident {
                    fn get_type() -> _pb::JsType {
                        _pb::JsType::Object(_pb::ObjectMeta {
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

#[cfg(test)]
mod test {
    use quote::quote;

    use crate::derive_js_implementation;

    #[test]
    fn test_macro() {
        let input_stream = quote!(
            #[derive(Serialize)]
            struct Test {
                name: String,
            }
        );

        let out = derive_js_implementation(input_stream.into());
        println!("{}", out.to_string())
    }
}
