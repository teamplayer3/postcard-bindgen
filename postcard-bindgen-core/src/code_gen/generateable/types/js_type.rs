use genco::prelude::js::Tokens;

use crate::{code_gen::generateable::VariablePath, type_info::ValueType};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for ValueType {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_ser_accessor(variable_path),
            Self::Array(array_meta) => array_meta.gen_ser_accessor(variable_path),
            Self::Object(object_meta) => object_meta.gen_ser_accessor(variable_path),
            Self::Optional(optional_meta) => optional_meta.gen_ser_accessor(variable_path),
            Self::String(string_meta) => string_meta.gen_ser_accessor(variable_path),
            Self::Range(range_meta) => range_meta.gen_ser_accessor(variable_path),
            Self::Map(map_meta) => map_meta.gen_ser_accessor(variable_path),
        }
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_des_accessor(field_accessor),
            Self::Array(array_meta) => array_meta.gen_des_accessor(field_accessor),
            Self::Object(object_meta) => object_meta.gen_des_accessor(field_accessor),
            Self::Optional(optional_meta) => optional_meta.gen_des_accessor(field_accessor),
            Self::String(string_meta) => string_meta.gen_des_accessor(field_accessor),
            Self::Range(range_meta) => range_meta.gen_des_accessor(field_accessor),
            Self::Map(map_meta) => map_meta.gen_des_accessor(field_accessor),
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
        }
    }

    fn gen_ts_type(&self) -> Tokens {
        match self {
            Self::Number(number_meta) => number_meta.gen_ts_type(),
            Self::Array(array_meta) => array_meta.gen_ts_type(),
            Self::Object(object_meta) => object_meta.gen_ts_type(),
            Self::Optional(optional_meta) => optional_meta.gen_ts_type(),
            Self::String(string_meta) => string_meta.gen_ts_type(),
            Self::Range(range_meta) => range_meta.gen_ts_type(),
            Self::Map(map_meta) => map_meta.gen_ts_type(),
        }
    }
}
