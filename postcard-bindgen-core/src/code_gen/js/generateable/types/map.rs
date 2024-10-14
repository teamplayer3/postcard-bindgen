use core::ops::Deref;

use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::{
        js::{FieldAccessor, VariablePath},
        utils::TokensIterExt,
    },
    type_info::{MapMeta, ValueType},
};

use super::JsTypeGenerateable;

impl JsTypeGenerateable for MapMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        match self.key_type.deref() {
            &ValueType::String(_) => {
                let inner_type_accessor = self.value_type.gen_ser_accessor(VariablePath::default());
                quote!(s.serialize_string_key_map((s, v) => $inner_type_accessor, $variable_path))
            }
            _ => {
                let inner_type_key_accessor = self
                    .key_type
                    .gen_ser_accessor(VariablePath::new("k".into()));
                let inner_type_value_accessor = self
                    .value_type
                    .gen_ser_accessor(VariablePath::new("v".into()));
                quote!(s.serialize_map((ser, k, v) => [$inner_type_key_accessor, $inner_type_value_accessor], $variable_path))
            }
        }
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        match self.key_type.deref() {
            &ValueType::String(_) => {
                let inner_type_accessor = self.value_type.gen_des_accessor(FieldAccessor::None);
                quote!($(field_accessor)d.deserialize_string_key_map(((d) => $inner_type_accessor)))
            }
            _ => {
                let inner_type_key_accessor = self.key_type.gen_des_accessor(FieldAccessor::None);
                let inner_type_value_accessor =
                    self.value_type.gen_des_accessor(FieldAccessor::None);
                quote!($(field_accessor)d.deserialize_map(((des) => [$inner_type_key_accessor, $inner_type_value_accessor])))
            }
        }
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        match self.key_type.deref() {
            &ValueType::String(_) => {
                let inner_type_check = self.value_type.gen_ty_check(VariablePath::new("v".into()));
                let inner_type_checks = quote!(Object.values($(variable_path.to_owned())).map((v) => $inner_type_check).every((v) => v));
                [
                    quote!(typeof $variable_path === "object"),
                    inner_type_checks,
                ]
                .into_iter()
                .join_logic_and()
            }
            _ => quote!($variable_path instanceof Map),
        }
    }

    fn gen_ts_type(&self) -> Tokens {
        match self.key_type.deref() {
            &ValueType::String(_) => {
                let value_type = self.value_type.gen_ts_type();
                quote!({
                    [key: string]: $value_type;
                })
            }
            _ => {
                let key_type = self.key_type.gen_ts_type();
                let value_type = self.value_type.gen_ts_type();
                quote!(Map<$key_type, $value_type>)
            }
        }
    }
}
