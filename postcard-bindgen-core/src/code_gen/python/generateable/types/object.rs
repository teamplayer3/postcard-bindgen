use genco::quote;

use crate::{
    code_gen::python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
    type_info::ObjectMeta,
    // utils::StrExt,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for ObjectMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        // let obj_ident = self.name.to_obj_identifier();
        let obj_ident = String::from(self.name);
        quote!(serialize_$obj_ident(s, $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        // let obj_ident = self.name.to_obj_identifier();
        let obj_ident = String::from(self.name);
        quote!($(field_accessor)deserialize_$obj_ident(d))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        // let obj_ident = self.name.to_obj_identifier();
        let obj_ident = String::from(self.name);
        quote!(assert_$obj_ident($variable_path))
    }

    fn gen_typings(&self, _import_registry: &mut ImportRegistry) -> Tokens {
        quote!($(self.name))
    }
}
