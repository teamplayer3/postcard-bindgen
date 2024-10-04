pub mod impls;

use genco::prelude::js::Tokens;

pub trait BindingTypeGenerateable {
    fn gen_ser_body(&self) -> Tokens;

    fn gen_des_body(&self) -> Tokens;

    fn gen_ty_check_body(&self) -> Tokens;

    fn gen_ts_typings_body(&self) -> Tokens;
}

mod ser {
    use genco::prelude::js::Tokens;

    use crate::{
        code_gen::js::{
            generateable::{types::JsTypeGenerateable, VariableAccess, VariablePath},
            utils::semicolon_chain,
        },
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_accessors_indexed(
        fields: impl AsRef<[ValueType]>,
        variable_path: VariablePath,
    ) -> Tokens {
        semicolon_chain(fields.as_ref().iter().enumerate().map(|(index, field)| {
            let path = variable_path
                .to_owned()
                .modify_push(VariableAccess::Indexed(index));
            field.gen_ser_accessor(path)
        }))
    }

    pub fn gen_accessors_fields(
        fields: impl AsRef<[StructField]>,
        variable_path: VariablePath,
    ) -> Tokens {
        semicolon_chain(fields.as_ref().iter().map(|field| {
            let path = variable_path
                .to_owned()
                .modify_push(VariableAccess::Field(field.name.into()));
            field.v_type.gen_ser_accessor(path)
        }))
    }
}

mod des {
    use genco::{prelude::js::Tokens, quote};

    use crate::{
        code_gen::js::{
            generateable::types::{self, JsTypeGenerateable},
            utils::comma_chain,
        },
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_accessors_fields(fields: impl AsRef<[StructField]>) -> Tokens {
        let body = comma_chain(fields.as_ref().iter().map(|field| {
            field
                .v_type
                .gen_des_accessor(types::des::FieldAccessor::Object(field.name))
        }));
        quote!({ $body })
    }

    pub fn gen_accessors_indexed(fields: impl AsRef<[ValueType]>) -> Tokens {
        let body = comma_chain(
            fields
                .as_ref()
                .iter()
                .enumerate()
                .map(|(_, v_type)| v_type.gen_des_accessor(types::des::FieldAccessor::Array)),
        );
        quote!([$body])
    }
}

mod ty_check {
    use genco::{prelude::js::Tokens, quote};

    use crate::{
        code_gen::js::{
            generateable::{types::JsTypeGenerateable, VariableAccess, VariablePath},
            utils::and_chain,
        },
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_object_checks(
        fields: impl AsRef<[StructField]>,
        variable_path: VariablePath,
    ) -> Tokens {
        let field_checks = and_chain(fields.as_ref().iter().map(|field| {
            let path = variable_path
                .to_owned()
                .modify_push(VariableAccess::Field(field.name.into()));
            field.v_type.gen_ty_check(path)
        }));
        quote!(typeof $variable_path === "object" && $field_checks)
    }

    pub fn gen_array_checks(fields: impl AsRef<[ValueType]>, variable_path: VariablePath) -> Tokens {
        let arr_len = fields.as_ref().len();
        let field_checks = and_chain(fields.as_ref().iter().enumerate().map(|(index, field)| {
            let path = variable_path
                .to_owned()
                .modify_push(VariableAccess::Indexed(index));
            field.gen_ty_check(path)
        }));
        quote!(Array.isArray($(variable_path.to_owned())) && $variable_path.length === $arr_len && $field_checks)
    }
}

pub mod ts {
    use genco::{prelude::js::Tokens, quote};

    use crate::{
        code_gen::js::{generateable::types::JsTypeGenerateable, utils::comma_chain},
        registry::StructField,
        type_info::ValueType,
    };

    pub fn gen_typings_indexed(fields: impl AsRef<[ValueType]>) -> Tokens {
        let body = comma_chain(fields.as_ref().iter().map(|f| quote!($(f.gen_ts_type()))));
        quote!([$body])
    }

    pub fn gen_typings_fields(fields: impl AsRef<[StructField]>) -> Tokens {
        let body = comma_chain(
            fields
                .as_ref()
                .iter()
                .map(|f| quote!($(f.name): $(f.v_type.gen_ts_type()))),
        );
        quote!({ $body })
    }
}
