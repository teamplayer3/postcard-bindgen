mod enums;
mod structs;
mod tuple_structs;
mod unit_structs;

use crate::{
    code_gen::python::{ImportRegistry, Tokens},
    registry::BindingType,
    utils::ContainerPath,
};

pub trait BindingTypeGenerateable {
    fn gen_ser_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens;

    fn gen_des_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens;

    fn gen_ty_check_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens;

    fn gen_typings_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens;
}

impl BindingTypeGenerateable for BindingType {
    fn gen_ser_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ser_body(name, path),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ser_body(name, path),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ser_body(name, path),
            Self::Enum(enum_type) => enum_type.gen_ser_body(name, path),
        }
    }

    fn gen_des_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_des_body(name, path),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_des_body(name, path),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_des_body(name, path),
            Self::Enum(enum_type) => enum_type.gen_des_body(name, path),
        }
    }

    fn gen_ty_check_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
    ) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ty_check_body(name, path),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ty_check_body(name, path),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ty_check_body(name, path),
            Self::Enum(enum_type) => enum_type.gen_ty_check_body(name, path),
        }
    }

    fn gen_typings_body<'a>(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<ContainerPath<'a>>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_typings_body(name, path, import_registry),
            Self::UnitStruct(unit_struct_type) => {
                unit_struct_type.gen_typings_body(name, path, import_registry)
            }
            Self::TupleStruct(tuple_struct_type) => {
                tuple_struct_type.gen_typings_body(name, path, import_registry)
            }
            Self::Enum(enum_type) => enum_type.gen_typings_body(name, path, import_registry),
        }
    }
}
