use crate::{
    code_gen::python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
    type_info::ValueType,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for ValueType {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_ser_accessor(variable_path),
            Self::Array(array_meta) => array_meta.gen_ser_accessor(variable_path),
            Self::Object(object_meta) => object_meta.gen_ser_accessor(variable_path),
            Self::Optional(optional_meta) => optional_meta.gen_ser_accessor(variable_path),
            Self::String(string_meta) => string_meta.gen_ser_accessor(variable_path),
            Self::Range(range_meta) => range_meta.gen_ser_accessor(variable_path),
            Self::Map(map_meta) => map_meta.gen_ser_accessor(variable_path),
            Self::Tuple(tuple_meta) => tuple_meta.gen_ser_accessor(variable_path),
        }
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_des_accessor(field_accessor),
            Self::Array(array_meta) => array_meta.gen_des_accessor(field_accessor),
            Self::Object(object_meta) => object_meta.gen_des_accessor(field_accessor),
            Self::Optional(optional_meta) => optional_meta.gen_des_accessor(field_accessor),
            Self::String(string_meta) => string_meta.gen_des_accessor(field_accessor),
            Self::Range(range_meta) => range_meta.gen_des_accessor(field_accessor),
            Self::Map(map_meta) => map_meta.gen_des_accessor(field_accessor),
            Self::Tuple(tuple_meta) => tuple_meta.gen_des_accessor(field_accessor),
        }
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_ty_check(variable_path),
            Self::Array(array_meta) => array_meta.gen_ty_check(variable_path),
            Self::Object(object_meta) => object_meta.gen_ty_check(variable_path),
            Self::Optional(optional_meta) => optional_meta.gen_ty_check(variable_path),
            Self::String(string_meta) => string_meta.gen_ty_check(variable_path),
            Self::Range(range_meta) => range_meta.gen_ty_check(variable_path),
            Self::Map(map_meta) => map_meta.gen_ty_check(variable_path),
            Self::Tuple(tuple_meta) => tuple_meta.gen_ty_check(variable_path),
        }
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_typings(import_registry),
            Self::Array(array_meta) => array_meta.gen_typings(import_registry),
            Self::Object(object_meta) => object_meta.gen_typings(import_registry),
            Self::Optional(optional_meta) => optional_meta.gen_typings(import_registry),
            Self::String(string_meta) => string_meta.gen_typings(import_registry),
            Self::Range(range_meta) => range_meta.gen_typings(import_registry),
            Self::Map(map_meta) => map_meta.gen_typings(import_registry),
            Self::Tuple(tuple_meta) => tuple_meta.gen_typings(import_registry),
        }
    }
}
