
mod js;

#[cfg(feature = "js")]
pub use js::{generate_js as generate, type_checking::ts::gen_ts_typings as generate_typings};