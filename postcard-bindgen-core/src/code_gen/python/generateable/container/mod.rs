mod enums;
mod structs;
mod tuple_structs;
mod unit_structs;

use crate::{
    code_gen::python::{ImportRegistry, Tokens},
    registry::{BindingType, ContainerInfo},
};

pub trait BindingTypeGenerateable {
    fn gen_ser_body(&self, container_info: ContainerInfo<'_>) -> Tokens;

    fn gen_des_body(&self, container_info: ContainerInfo<'_>) -> Tokens;

    fn gen_ty_check_body(&self, container_info: ContainerInfo<'_>) -> Tokens;

    fn gen_typings_body(
        &self,
        container_info: ContainerInfo<'_>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens;
}

impl BindingTypeGenerateable for BindingType {
    fn gen_ser_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ser_body(container_info),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ser_body(container_info),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ser_body(container_info),
            Self::Enum(enum_type) => enum_type.gen_ser_body(container_info),
        }
    }

    fn gen_des_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_des_body(container_info),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_des_body(container_info),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_des_body(container_info),
            Self::Enum(enum_type) => enum_type.gen_des_body(container_info),
        }
    }

    fn gen_ty_check_body(&self, container_info: ContainerInfo<'_>) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ty_check_body(container_info),
            Self::UnitStruct(unit_struct_type) => {
                unit_struct_type.gen_ty_check_body(container_info)
            }
            Self::TupleStruct(tuple_struct_type) => {
                tuple_struct_type.gen_ty_check_body(container_info)
            }
            Self::Enum(enum_type) => enum_type.gen_ty_check_body(container_info),
        }
    }

    fn gen_typings_body(
        &self,
        container_info: ContainerInfo<'_>,
        import_registry: &mut ImportRegistry,
    ) -> Tokens {
        match self {
            Self::Struct(struct_type) => {
                struct_type.gen_typings_body(container_info, import_registry)
            }
            Self::UnitStruct(unit_struct_type) => {
                unit_struct_type.gen_typings_body(container_info, import_registry)
            }
            Self::TupleStruct(tuple_struct_type) => {
                tuple_struct_type.gen_typings_body(container_info, import_registry)
            }
            Self::Enum(enum_type) => enum_type.gen_typings_body(container_info, import_registry),
        }
    }
}
