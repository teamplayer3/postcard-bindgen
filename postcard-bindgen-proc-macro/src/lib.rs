use generate_js::gen_ser_der_funcs;
use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{
    ast::{self, Style},
    Ctxt, Derive,
};
use syn::DeriveInput;

mod generate_js;

#[proc_macro_derive(PostcardBindings)]
pub fn postcard_bindings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_js_implementation(input).into()
}

fn derive_js_implementation(input: proc_macro::TokenStream) -> TokenStream {
    // eprintln!(".........[input] {}", input);
    let input: DeriveInput = syn::parse(input).unwrap();

    let cx = Ctxt::new();
    let container = ast::Container::from_ast(&cx, &input, Derive::Serialize).unwrap();

    let typescript = match container.data {
        ast::Data::Enum(_) => unimplemented!(),
        ast::Data::Struct(style, fields) => match style {
            Style::Struct => gen_ser_der_funcs(container.ident.to_string(), &fields),
            _ => unimplemented!(),
        },
    };

    let typescript_string = typescript.to_string().unwrap();
    let container_ident = container.ident;
    let container_ident_string = container_ident.to_string();
    let container_ident_str = container_ident_string.as_str();

    let expanded = if cfg!(any(debug_assertions, feature = "export-js")) {
        quote! {
            impl postcard_bindgen::JsExportable for #container_ident {
                const JS_STRING : &'static str = #typescript_string;
                const TYPE_IDENT: &'static str = #container_ident_str;
            }
        }
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
