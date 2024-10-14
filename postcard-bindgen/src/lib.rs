//! # Postcard Bindgen
//!
//! This crate allows automatically generating javascript bindings to
//! serialize javascript objects to postcard format and vice versa.
//!
//! # Example
//!
//! This example shows how to generate a `npm` package out of the rust
//! structures. A new folder with the package name will be created. A
//! javascript file and typescript typings as well as a package.json
//! will be placed in it.
//!
//! ```rust
//! # use postcard_bindgen::{PostcardBindings, generate_bindings, build_package, PackageInfo};
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
//!     build_package(
//!         std::env::current_dir().unwrap().as_path(),
//!         PackageInfo {
//!             name: "test-bindings".into(),
//!             version: "0.1.0".try_into().unwrap(),
//!         },
//!         generate_bindings!(A, B, C),
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

#![cfg_attr(not(feature = "generating"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "generating")]
#[cfg_attr(docsrs, doc(cfg(feature = "generating")))]
mod package;

#[cfg(feature = "generating")]
#[cfg_attr(docsrs, doc(cfg(feature = "generating")))]
pub mod javascript {
    pub use super::package::npm_package::build_npm_package as build_package;
    pub use postcard_bindgen_core::code_gen::js::GenerationSettings;
}

#[cfg(feature = "generating")]
#[cfg_attr(docsrs, doc(cfg(feature = "generating")))]
pub mod python {
    pub use super::package::pip_module::build_pip_module as build_package;
    pub use postcard_bindgen_core::code_gen::python::GenerationSettings;
}

#[cfg(feature = "generating")]
#[cfg_attr(docsrs, doc(cfg(feature = "generating")))]
pub use package::{PackageInfo, Version, VersionFromStrError};

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

#[cfg(feature = "generating")]
#[doc(hidden)]
pub mod __private {
    pub use postcard_bindgen_core::{
        registry::*,
        type_info::{GenJsBinding, ObjectMeta, ValueType},
    };
}

/// Macro to generate javascript and typescript binding strings which
/// can be exported into files.
///
/// The supplied structures needs to implement the `trait` [`crate::__private::JsBindings`].
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
#[cfg_attr(docsrs, doc(cfg(feature = "generating")))]
#[macro_export]
macro_rules! generate_bindings {
    ($( $x:ty ),*) => {
        {
            let mut reg = postcard_bindgen::__private::BindingsRegistry::default();
            $(
                <$x as postcard_bindgen::__private::JsBindings>::create_bindings(&mut reg);
            )*
            reg.into_entries()
        }
    };
}
