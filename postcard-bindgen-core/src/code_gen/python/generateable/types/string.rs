use genco::quote;

use crate::{
    code_gen::python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
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
        quote!(assert isinstance($(variable_path.to_owned()), str), "{} is not a string".format($variable_path))
    }

    fn gen_typings(&self, _import_registry: &mut ImportRegistry) -> Tokens {
        quote!(str)
    }
}
