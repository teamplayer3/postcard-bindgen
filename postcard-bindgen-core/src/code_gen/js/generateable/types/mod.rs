pub mod array;
mod bool;
pub mod map;
pub mod number;
pub mod object;
pub mod optional;
pub mod range;
pub mod string;
pub mod tuple;

pub mod js_type;

use genco::prelude::js::Tokens;

use crate::code_gen::js::{FieldAccessor, VariablePath};

pub trait JsTypeGenerateable {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens;

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens;

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens;

    fn gen_ts_type(&self) -> Tokens;
}
