use genco::quote;

use crate::{
    code_gen::{
        import_registry::{ImportItem, Package},
        python::{FieldAccessor, ImportRegistry, Tokens, VariableAccess, VariablePath},
        utils::TokensIterExt,
    },
    type_info::TupleMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for TupleMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        self.items_types
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.gen_ser_accessor(
                    variable_path
                        .to_owned()
                        .modify_push(VariableAccess::Indexed(i)),
                )
            })
            .join_with_line_breaks()
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_type_accessors = self
            .items_types
            .iter()
            .map(|v| v.gen_des_accessor(FieldAccessor::None))
            .join_with_comma();
        quote!($field_accessor($inner_type_accessors))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let type_checks = self
            .items_types
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.gen_ty_check(
                    variable_path
                        .to_owned()
                        .modify_push(VariableAccess::Indexed(i)),
                )
            })
            .join_with_line_breaks();
        [
            quote!(assert isinstance($(variable_path.to_owned()), tuple), "{} is not a tuple".format($(variable_path.to_owned()))),
            quote!(assert len($(variable_path.to_owned())) == $(self.items_types.len()), "{} is not of length {}".format($variable_path, $(self.items_types.len()))),
            type_checks,
        ]
        .into_iter()
        .join_with_line_breaks()
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        let type_checks = self
            .items_types
            .iter()
            .map(|v| v.gen_typings(import_registry))
            .join_with_comma();
        import_registry.push(
            Package::Extern("typing".into()),
            ImportItem::Single("Tuple".into()),
        );
        quote!(Tuple[$type_checks])
    }
}
