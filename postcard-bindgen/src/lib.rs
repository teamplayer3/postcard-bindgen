pub use postcard_bindgen_core::{
    build_npm_package, export_bindings, gen_ts_typings, generate_js,
    registry::*,
    type_info::{GenJsBinding, JsType, ObjectMeta},
    ArchPointerLen, ExportStrings, PacketInfo,
};
pub use postcard_bindgen_proc_macro::PostcardBindings;

/// Macro to generate javascript and typescript binding strings which
/// can be exported into files.
///
/// The supplied structures needs to implement the `trait` [`crate::JsBindings`].
/// This `trait` is automatically implemented when deriving the
/// [`postcard_bindgen::PostcardBindings`] on the types.
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
            let mut reg = postcard_bindgen::BindingsRegistry::default();
            $(
                <$x as postcard_bindgen::JsBindings>::create_bindings(&mut reg);
            )*
            let bindings = reg.into_entries();
            postcard_bindgen::ExportStrings {
                js_file: postcard_bindgen::generate_js(&bindings).to_file_string().unwrap(),
                ts_file: postcard_bindgen::gen_ts_typings(bindings).to_file_string().unwrap()
            }
        }
    };
}
