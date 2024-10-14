use genco::quote;

use crate::{
    code_gen::{
        python::{FieldAccessor, ImportRegistry, Tokens, VariablePath, PYTHON_OBJECT_VARIABLE},
        utils::TokensIterExt,
    },
    type_info::ArrayMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for ArrayMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let inner_type_accessor = self.items_type.gen_ser_accessor(VariablePath::default());
        if let Some(len) = self.length {
            quote!(s.serialize_array(lambda s, $PYTHON_OBJECT_VARIABLE: $inner_type_accessor, $variable_path, $len))
        } else {
            quote!(s.serialize_array(lambda s, $PYTHON_OBJECT_VARIABLE: $inner_type_accessor, $variable_path, None))
        }
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_type_accessor = self.items_type.gen_des_accessor(FieldAccessor::Array);
        if let Some(len) = self.length {
            quote!($(field_accessor)d.deserialize_array(lambda d: $inner_type_accessor, $len))
        } else {
            quote!($(field_accessor)d.deserialize_array(lambda d: $inner_type_accessor, None))
        }
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let assert_item_type_check_func = quote! {
            def assert_$(variable_path.to_owned().to_string("_"))($PYTHON_OBJECT_VARIABLE):
                $(self.items_type.gen_ty_check(VariablePath::default()))
        };
        let item_ty_check = quote!([assert_$(variable_path.to_owned().to_string("_"))($PYTHON_OBJECT_VARIABLE) for $PYTHON_OBJECT_VARIABLE in $(variable_path.clone())]);
        if let Some(len) = self.length {
            [
                quote!(assert isinstance($(variable_path.to_owned()), list), "{} is not a list".format($(variable_path.to_owned()))),
                quote!(assert len($(variable_path.to_owned())) == $len, "{} has not a length of {}".format($(variable_path.to_owned()), $len)),
                assert_item_type_check_func,
                quote!($item_ty_check),
            ]
            .into_iter()
            .join_with_line_breaks()
        } else {
            [
                quote!(assert isinstance($(variable_path.to_owned()), list), "{} is not a list".format($(variable_path.to_owned()))),
                assert_item_type_check_func,
                quote!($item_ty_check),
            ]
            .into_iter()
            .join_with_line_breaks()
        }
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        quote!(list[$(self.items_type.gen_typings(import_registry))])
    }
}
