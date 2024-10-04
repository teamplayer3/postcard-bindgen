mod utils;

#[cfg(feature = "js")]
mod js;

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "js")]
pub use js::{generate_js as generate, type_checking::ts::gen_ts_typings as generate_typings};

#[cfg(feature = "python")]
pub use python::{generate, type_checking::typing::generate_typings};