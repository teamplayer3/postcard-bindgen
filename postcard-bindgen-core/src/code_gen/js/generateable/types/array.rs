use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::js::{FieldAccessor, VariablePath, JS_OBJECT_VARIABLE},
    type_info::ArrayMeta,
};

use super::JsTypeGenerateable;

impl JsTypeGenerateable for ArrayMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let inner_type_accessor = self.items_type.gen_ser_accessor(VariablePath::default());
        let helper_func_name = quote!(lambda_$(variable_path.to_owned().into_string("_")));
        let helper_func = quote! {
            const $(&helper_func_name) = (s, $JS_OBJECT_VARIABLE) => {
                $inner_type_accessor
            };
        };

        if let Some(len) = self.length {
            quote! {
                $helper_func
                s.serialize_array($helper_func_name, $variable_path, $len)
            }
        } else {
            quote! {
                $helper_func
                s.serialize_array($helper_func_name, $variable_path)
            }
        }
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_type_accessor = self.items_type.gen_des_accessor(FieldAccessor::Array);
        if let Some(len) = self.length {
            quote!($(field_accessor)d.deserialize_array(() => $inner_type_accessor, $len))
        } else {
            quote!($(field_accessor)d.deserialize_array(() => $inner_type_accessor))
        }
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let item_ty_check = quote!($(variable_path.clone()).every(($JS_OBJECT_VARIABLE) => $(self.items_type.gen_ty_check(VariablePath::default()))));
        if let Some(len) = self.length {
            quote!(Array.isArray($(variable_path.clone())) && $item_ty_check && $variable_path.length === $len)
        } else if let Some(len) = self.max_length {
            quote!(Array.isArray($(variable_path.clone())) && $item_ty_check && $variable_path.length <= $len)
        } else {
            quote!(Array.isArray($variable_path) && $item_ty_check)
        }
    }

    fn gen_ts_type(&self) -> Tokens {
        if let Some(len) = self.length {
            quote!(FixedLengthArray<$(self.items_type.gen_ts_type()), $len>)
        } else {
            quote!($(self.items_type.gen_ts_type())[])
        }
    }
}
