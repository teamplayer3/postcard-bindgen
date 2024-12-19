mod enums;
mod structs;
mod tuple_structs;
mod unit_structs;

use genco::prelude::js::Tokens;

use crate::registry::BindingType;

pub trait BindingTypeGenerateable {
    fn gen_ser_body(&self) -> Tokens;

    fn gen_des_body(&self) -> Tokens;

    fn gen_ty_check_body(&self) -> Tokens;

    fn gen_ts_typings_body(&self) -> Tokens;
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

    fn gen_ts_typings_body(&self) -> Tokens {
        match self {
            Self::Struct(struct_type) => struct_type.gen_ts_typings_body(),
            Self::UnitStruct(unit_struct_type) => unit_struct_type.gen_ts_typings_body(),
            Self::TupleStruct(tuple_struct_type) => tuple_struct_type.gen_ts_typings_body(),
            Self::Enum(enum_type) => enum_type.gen_ts_typings_body(),
        }
    }
}

mod ser {
    use genco::prelude::js::Tokens;

    use crate::{
        code_gen::{
            js::{generateable::types::JsTypeGenerateable, VariableAccess, VariablePath},
            utils::{JoinType, TokensIterExt},
        },
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_accessors_indexed(
        fields: impl AsRef<[ValueType]>,
        variable_path: VariablePath,
    ) -> Tokens {
        let mut body = fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let path = variable_path
                    .to_owned()
                    .modify_push(VariableAccess::Indexed(index));
                field.gen_ser_accessor(path)
            })
            .join_with([JoinType::Semicolon, JoinType::LineBreak]);

        body.append(";");
        body
    }

    pub fn gen_accessors_fields(
        fields: impl AsRef<[StructField]>,
        variable_path: VariablePath,
    ) -> Tokens {
        let mut body = fields
            .as_ref()
            .iter()
            .map(|field| {
                let path = variable_path
                    .to_owned()
                    .modify_push(VariableAccess::Field(field.name.into()));
                field.v_type.gen_ser_accessor(path)
            })
            .join_with([JoinType::Semicolon, JoinType::LineBreak]);

        body.append(";");
        body
    }
}

mod des {
    use genco::{prelude::js::Tokens, quote};

    use crate::{
        code_gen::{
            js::{generateable::types::JsTypeGenerateable, FieldAccessor},
            utils::{JoinType, TokensIterExt},
        },
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_accessors_fields(fields: impl AsRef<[StructField]>) -> Tokens {
        let body = fields
            .as_ref()
            .iter()
            .map(|field| {
                field
                    .v_type
                    .gen_des_accessor(FieldAccessor::Object(field.name))
            })
            .join_with([JoinType::Comma, JoinType::LineBreak]);
        quote! {
            {
                $body
            }
        }
    }

    pub fn gen_accessors_indexed(fields: impl AsRef<[ValueType]>) -> Tokens {
        let body = fields
            .as_ref()
            .iter()
            .map(|v_type| v_type.gen_des_accessor(FieldAccessor::Array))
            .join_with([JoinType::Comma, JoinType::LineBreak]);
        quote! {
            [
                $body
            ]
        }
    }
}

mod ty_check {
    use genco::{prelude::js::Tokens, quote};

    use crate::{
        code_gen::{
            js::{generateable::types::JsTypeGenerateable, VariableAccess, VariablePath},
            utils::TokensIterExt,
        },
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_object_checks(
        fields: impl AsRef<[StructField]>,
        variable_path: VariablePath,
    ) -> Tokens {
        let field_checks = fields
            .as_ref()
            .iter()
            .map(|field| {
                let path = variable_path
                    .to_owned()
                    .modify_push(VariableAccess::Field(field.name.into()));
                field.v_type.gen_ty_check(path)
            })
            .join_logic_and();
        quote!(typeof $variable_path === "object" && $field_checks)
    }

    pub fn gen_array_checks(
        fields: impl AsRef<[ValueType]>,
        variable_path: VariablePath,
    ) -> Tokens {
        let arr_len = fields.as_ref().len();
        let field_checks = fields
            .as_ref()
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let path = variable_path
                    .to_owned()
                    .modify_push(VariableAccess::Indexed(index));
                field.gen_ty_check(path)
            })
            .join_logic_and();

        quote!(Array.isArray($(variable_path.to_owned())) && $variable_path.length === $arr_len && $field_checks)
    }
}

pub mod ts {
    use genco::{prelude::js::Tokens, quote};

    use crate::{
        code_gen::{js::generateable::types::JsTypeGenerateable, utils::TokensIterExt},
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_typings_indexed(fields: impl AsRef<[ValueType]>) -> Tokens {
        let body = fields
            .as_ref()
            .iter()
            .map(|f| quote!($(f.gen_ts_type())))
            .join_with_comma();
        quote!([$body])
    }

    pub fn gen_typings_fields(fields: impl AsRef<[StructField]>) -> Tokens {
        let body = fields
            .as_ref()
            .iter()
            .map(|f| quote!($(f.name): $(f.v_type.gen_ts_type())))
            .join_with_comma();
        quote!({ $body })
    }
}
