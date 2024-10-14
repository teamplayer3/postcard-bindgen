use genco::quote;

use crate::{
    code_gen::{
        python::{FieldAccessor, ImportRegistry, Tokens, VariableAccess, VariablePath},
        utils::TokensIterExt,
    },
    type_info::RangeMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for RangeMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let start_path = variable_path
            .to_owned()
            .modify_push(VariableAccess::Field("start".to_owned()));
        let stop_path = variable_path.modify_push(VariableAccess::Field("stop".to_owned()));

        let start_accessor = self.bounds_type.gen_ser_accessor(start_path);
        let stop_accessor = self.bounds_type.gen_ser_accessor(stop_path);

        [start_accessor, stop_accessor]
            .into_iter()
            .join_with_line_breaks()
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let field_des = self.bounds_type.gen_des_accessor(FieldAccessor::None);
        quote!($field_accessor range($(field_des.to_owned()), $field_des))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        [
            quote!(assert isinstance($(variable_path.to_owned()), range), "{} is not a range".format($(variable_path.to_owned()))),
            self.bounds_type.gen_ty_check(
                variable_path
                    .to_owned()
                    .modify_push(VariableAccess::Field("start".to_owned())),
            ),
            self.bounds_type.gen_ty_check(
                variable_path
                    .to_owned()
                    .modify_push(VariableAccess::Field("stop".to_owned())),
            ),
        ]
        .into_iter()
        .join_with_line_breaks()
    }

    fn gen_typings(&self, _import_registry: &mut ImportRegistry) -> Tokens {
        quote!(range)
    }
}
