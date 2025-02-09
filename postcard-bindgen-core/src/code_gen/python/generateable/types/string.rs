use genco::quote;

use crate::{
    code_gen::{
        python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
        utils::TokensIterExt,
    },
    type_info::StringMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for StringMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        quote!(s.serialize_string($variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        quote!($(field_accessor)d.deserialize_string())
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let mut checks = vec![];
        checks.push(quote!(assert isinstance($(variable_path.to_owned()), str), "{} is not a string".format($(variable_path.to_owned()))));
        if let Some(len) = self.max_length {
            checks.push(quote!(assert len($(variable_path.to_owned())) <= $len, "{} has a length greater than {}".format($variable_path, $len)));
        }
        checks.into_iter().join_with_line_breaks()
    }

    fn gen_typings(&self, _import_registry: &mut ImportRegistry) -> Tokens {
        quote!(str)
    }
}
