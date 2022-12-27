//! # Postcard Bindgen
//!
//! This crate allows generating javascript bindings automatically to
//! serialize javascript objects to postcard format and vice versa.
//!
//! # Example
//!
//! This example shows how to generate a `npm` package out of the rust
//! structures. A new folder with the package name will be created. A
//! javascript file and typescript typings as well as a package.json
//! will be paced in it.
//!
//! ```rust
//! # use postcard_bindgen::{PostcardBindings, generate_bindings, build_npm_package, PackageInfo};
//! # use serde::Serialize;
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
//! ```
//!
//! ```text
//! // JavaScript
//! import { serialize } form "test-bindings"
//!
//! const c = {
//!     tag: "C",
//!     value: [
//!         123,
//!         {
//!             a: 234
//!         }
//!     ]
//! }
//!
//! const bytes = serialize("C", c)
//! ```

#[cfg(feature = "generating")]
use doc_cfg::doc_cfg;
#[cfg(feature = "generating")]
mod export;
#[cfg(feature = "generating")]
mod npm_package;

#[cfg(feature = "generating")]
#[doc_cfg(feature = "generating")]
pub use export::export_bindings;
#[cfg(feature = "generating")]
#[doc_cfg(feature = "generating")]
pub use npm_package::{build_npm_package, PackageInfo, Version, VersionFromStrError};
#[cfg(feature = "generating")]
#[doc_cfg(feature = "generating")]
pub use postcard_bindgen_core::ExportStrings;

pub use postcard_bindgen_core::type_info::GenJsBinding;

/// Macro to annotate structs or enums for which bindings should be generated.
///
/// For this macro to work, the [`serde::Serialize`] macro must be derived as well.
///
/// # Example
/// ```rust
/// # use serde::Serialize;
/// # use postcard_bindgen_derive::PostcardBindings;
/// #[derive(Serialize, PostcardBindings)]
/// struct Test {
///    a: u32
/// }
/// ```
pub use postcard_bindgen_derive::PostcardBindings;

#[doc(hidden)]
pub mod __private {
    pub use postcard_bindgen_core::{
        gen_ts_typings, generate_js,
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
/// ```rust
/// # use serde::Serialize;
/// # use postcard_bindgen::{PostcardBindings, generate_bindings};
/// #[derive(Serialize, PostcardBindings)]
/// struct Test {
///     field: u8
/// }
///
/// let bindings = generate_bindings!(Test);
/// ```
#[cfg(feature = "generating")]
#[macro_export]
macro_rules! generate_bindings {
    ($( $x:ty ),*) => {
        {
            let mut reg = postcard_bindgen::__private::BindingsRegistry::default();
            $(
                <$x as postcard_bindgen::__private::JsBindings>::create_bindings(&mut reg);
            )*
            let bindings = reg.into_entries();
            postcard_bindgen::ExportStrings {
                js_file: postcard_bindgen::__private::generate_js(&bindings).to_file_string().unwrap(),
                ts_file: postcard_bindgen::__private::gen_ts_typings(bindings).to_file_string().unwrap()
            }
        }
    };
}
