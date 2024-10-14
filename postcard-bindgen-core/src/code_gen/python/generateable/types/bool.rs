use genco::quote;

use crate::{
    code_gen::python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
    type_info::BoolMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for BoolMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        quote!(s.serialize_bool($variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        quote!($(field_accessor)d.deserialize_bool())
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        quote!(assert isinstance($(variable_path.to_owned()), int), "{} is not a bool".format($variable_path))
    }

    fn gen_typings(&self, _import_registry: &mut ImportRegistry) -> Tokens {
        quote!(bool)
    }
}

pub fn bool_to_python_bool(value: bool) -> &'static str {
    if value {
        "True"
    } else {
        "False"
    }
}
