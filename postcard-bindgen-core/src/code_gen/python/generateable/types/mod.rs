use crate::code_gen::python::{FieldAccessor, ImportRegistry, Tokens, VariablePath};

mod array;
mod bool;
mod map;
mod number;
mod object;
mod optional;
mod python_type;
mod range;
mod string;
mod tuple;

pub trait PythonTypeGenerateable {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens;

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens;

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens;

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens;
}
