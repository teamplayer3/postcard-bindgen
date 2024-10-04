use genco::{lang::js::Tokens, quote};

use crate::{
    code_gen::{
        js::generateable::{types::des::FieldAccessor, VariableAccess, VariablePath},
        utils::{comma_chain, semicolon_chain},
    },
    type_info::TupleMeta,
};

use super::{des, JsTypeGenerateable};

impl JsTypeGenerateable for TupleMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        semicolon_chain(self.items_types.iter().enumerate().map(|(i, v)| {
            v.gen_ser_accessor(
                variable_path
                    .clone()
                    .modify_push(VariableAccess::Indexed(i)),
            )
        }))
    }

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens {
        let inner_type_accessors = comma_chain(
            self.items_types
                .iter()
                .map(|v| v.gen_des_accessor(FieldAccessor::None)),
        );
        quote!($field_accessor[$inner_type_accessors])
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let type_checks = comma_chain(self.items_types.iter().enumerate().map(|(i, v)| {
            v.gen_ty_check(
                variable_path
                    .clone()
                    .modify_push(VariableAccess::Indexed(i)),
            )
        }));
        quote!(Array.isArray($(variable_path.clone())) && $variable_path.length === $(self.items_types.len()) && $type_checks)
    }

    fn gen_ts_type(&self) -> Tokens {
        let type_checks = comma_chain(self.items_types.iter().map(|v| v.gen_ts_type()));
        quote!([$type_checks])
    }
}
