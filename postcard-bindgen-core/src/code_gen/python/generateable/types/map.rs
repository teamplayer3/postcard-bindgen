use genco::quote;

use crate::{
    code_gen::{
        python::{FieldAccessor, ImportRegistry, Tokens, VariablePath},
        utils::TokensIterExt,
    },
    type_info::MapMeta,
};

use super::PythonTypeGenerateable;

impl PythonTypeGenerateable for MapMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let inner_type_key_accessor = self
            .key_type
            .gen_ser_accessor(VariablePath::new("k".into()));
        let inner_type_value_accessor = self
            .value_type
            .gen_ser_accessor(VariablePath::new("v".into()));
        quote!(s.serialize_map(lambda s, k, v: ($inner_type_key_accessor, $inner_type_value_accessor), $variable_path))
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_type_key_accessor = self.key_type.gen_des_accessor(FieldAccessor::None);
        let inner_type_value_accessor = self.value_type.gen_des_accessor(FieldAccessor::None);
        quote!($(field_accessor)d.deserialize_map((lambda d: ($inner_type_key_accessor, $inner_type_value_accessor))))
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let assert_func_name = quote!(assert_$(variable_path.to_owned().into_string("_")));
        let assert_item_type_check_func = quote! {
            def $(&assert_func_name)(key, value):
                $(self.key_type.gen_ty_check(VariablePath::new("key".to_owned())))
                $(self.value_type.gen_ty_check(VariablePath::new("value".to_owned())))
        };
        let item_ty_check = quote!([$assert_func_name(key, value) for key, value in $(variable_path.to_owned()).items()]);

        [
            quote!(assert isinstance($(variable_path.to_owned()), dict), "{} is not a dict".format($variable_path)),
            assert_item_type_check_func,
            item_ty_check

        ]
        .into_iter()
        .join_with_line_breaks()
    }

    fn gen_typings(&self, import_registry: &mut ImportRegistry) -> Tokens {
        let key_type = self.key_type.gen_typings(import_registry);
        let value_type = self.value_type.gen_typings(import_registry);
        quote!(dict[$key_type, $value_type])
    }
}
