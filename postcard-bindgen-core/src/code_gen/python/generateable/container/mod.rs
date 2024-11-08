mod enums;
mod structs;
mod tuple_structs;
mod unit_structs;

use crate::{
    code_gen::python::{ImportRegistry, Tokens},
    registry::BindingType,
};

pub trait BindingTypeGenerateable {
    fn gen_ser_body(&self, name: impl AsRef<str>) -> Tokens;

    fn gen_des_body(&self, name: impl AsRef<str>) -> Tokens;

    fn gen_ty_check_body(&self, name: impl AsRef<str>) -> Tokens;

    fn gen_typings_body(
        &self,
        name: impl AsRef<str>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens;
}

impl BindingTypeGenerateable for BindingType {
    fn gen_ser_body(&self, name: impl AsRef<str>) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ser_body(name),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ser_body(name),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ser_body(name),
            Self::Enum(enum_type) => enum_type.gen_ser_body(name),
        }
    }

    fn gen_des_body(&self, name: impl AsRef<str>) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_des_body(name),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_des_body(name),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_des_body(name),
            Self::Enum(enum_type) => enum_type.gen_des_body(name),
        }
    }

    fn gen_ty_check_body(&self, name: impl AsRef<str>) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ty_check_body(name),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ty_check_body(name),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ty_check_body(name),
            Self::Enum(enum_type) => enum_type.gen_ty_check_body(name),
        }
    }

    fn gen_typings_body(
        &self,
        name: impl AsRef<str>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_typings_body(name, import_registry),
            Self::UnitStruct(unit_struct_type) => {
                unit_struct_type.gen_typings_body(name, import_registry)
            }
            Self::TupleStruct(tuple_struct_type) => {
                tuple_struct_type.gen_typings_body(name, import_registry)
            }
            Self::Enum(enum_type) => enum_type.gen_typings_body(name, import_registry),
        }
    }
}
