mod enums;
mod structs;
mod tuple_structs;
mod unit_struct_types;

use crate::{
    code_gen::python::{ImportRegistry, Tokens},
    registry::BindingType,
};

pub trait BindingTypeGenerateable {
    fn gen_ser_body(&self) -> Tokens;

    fn gen_des_body(&self) -> Tokens;

    fn gen_ty_check_body(&self) -> Tokens;

    fn gen_typings_body(&self, import_registry: &mut ImportRegistry) -> Tokens;
}

impl BindingTypeGenerateable for BindingType {
    fn gen_ser_body(&self) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ser_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ser_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ser_body(),
            Self::Enum(enum_type) => enum_type.gen_ser_body(),
        }
    }

    fn gen_des_body(&self) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_des_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_des_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_des_body(),
            Self::Enum(enum_type) => enum_type.gen_des_body(),
        }
    }

    fn gen_ty_check_body(&self) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ty_check_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ty_check_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ty_check_body(),
            Self::Enum(enum_type) => enum_type.gen_ty_check_body(),
        }
    }

    fn gen_typings_body(&self, import_registry: &mut ImportRegistry) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_typings_body(import_registry),
            Self::UnitStruct(unit_struct_type) => {
                unit_struct_type.gen_typings_body(import_registry)
            }
            Self::TupleStruct(tuple_struct_type) => {
                tuple_struct_type.gen_typings_body(import_registry)
            }
            Self::Enum(enum_type) => enum_type.gen_typings_body(import_registry),
        }
    }
}
