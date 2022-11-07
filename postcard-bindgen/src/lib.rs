//! # Postcard Bindgen
//!
//! This crate allows generating automatically javascript bindings to
//! serialize javascript objects to postcard format and vice versa.
//!
//! # Example
//! ```
//! # use serde_derive::Serialize;
//! # use postcard_bindgen::{PostcardBindings, generate_bindings, build_npm_package, PackageInfo};
//! # extern crate alloc;
//! #[derive(Serialize, PostcardBindings)]
//! struct A(u8);
//!
//! #[derive(Serialize, PostcardBindings)]
//! struct B {
//!     a: u8
//! }
//!
//! #[derive(Serialize, PostcardBindings)]
//! enum C {
//!     A,
//!     B(u8),
//!     C(A, B),
//!     D { a: &'static str, b: B },
//! }
//!
//! fn main() {
//!     build_npm_package(
//!         std::env::current_dir().unwrap().as_path(),
//!         PackageInfo {
//!             name: "test-bindings".into(),
//!             version: "0.1.0".try_into().unwrap(),
//!         },
//!         generate_bindings!(A, B),
//!     )
//!     .unwrap();
//! }

pub use postcard_bindgen_core::{
    build_npm_package, export_bindings, gen_ts_typings, generate_js, ExportStrings, PackageInfo,
    Version,
};
pub use postcard_bindgen_derive::PostcardBindings;

#[doc(hidden)]
pub mod private {
    pub use postcard_bindgen_core::{
        registry::*,
        type_info::{GenJsBinding, JsType, ObjectMeta},
    };
}

/// Macro to generate javascript and typescript binding strings which
/// can be exported into files.
///
/// The supplied structures needs to implement the `trait` [`crate::private::JsBindings`].
/// This `trait` is automatically implemented when deriving the
/// [`postcard_bindgen_derive::PostcardBindings`] on the types.
///
/// # Example
/// ```ignore
/// #[derive(Serialize, PostcardBindings)]
/// struct Test {
///     field: u8
/// }
///
/// let bindings = generate_bindings!(Test);
/// ```
#[macro_export]
macro_rules! generate_bindings {
    ($( $x:ty ),*) => {
        {
            let mut reg = postcard_bindgen::private::BindingsRegistry::default();
            $(
                <$x as postcard_bindgen::private::JsBindings>::create_bindings(&mut reg);
            )*
            let bindings = reg.into_entries();
            postcard_bindgen::ExportStrings {
                js_file: postcard_bindgen::generate_js(&bindings).to_file_string().unwrap(),
                ts_file: postcard_bindgen::gen_ts_typings(bindings).to_file_string().unwrap()
            }
        }
    };
}
