use genco::{lang::js::Tokens, quote};

use crate::{
    code_gen::{
        js::{FieldAccessor, VariableAccess, VariablePath},
        utils::TokensIterExt,
    },
    type_info::TupleMeta,
};

use super::JsTypeGenerateable;

impl JsTypeGenerateable for TupleMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        self.items_types
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.gen_ser_accessor(
                    variable_path
                        .clone()
                        .modify_push(VariableAccess::Indexed(i)),
                )
            })
            .join_with_semicolon()
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_type_accessors = self
            .items_types
            .iter()
            .map(|v| v.gen_des_accessor(FieldAccessor::None))
            .join_with_comma();
        quote!($field_accessor[$inner_type_accessors])
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let type_checks = self
            .items_types
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.gen_ty_check(
                    variable_path
                        .clone()
                        .modify_push(VariableAccess::Indexed(i)),
                )
            })
            .join_with_comma();
        quote!(Array.isArray($(variable_path.clone())) && $variable_path.length === $(self.items_types.len()) && $type_checks)
    }

    fn gen_ts_type(&self) -> Tokens {
        let type_checks = self
            .items_types
            .iter()
            .map(|v| v.gen_ts_type())
            .join_with_comma();
        quote!([$type_checks])
    }
}
