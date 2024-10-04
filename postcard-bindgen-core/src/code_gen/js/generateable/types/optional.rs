use genco::{prelude::js::Tokens, quote};

use crate::{
    code_gen::js::{FieldAccessor, VariablePath},
    type_info::OptionalMeta,
};

use super::{
    ty_check::{self, AvailableCheck},
    JsTypeGenerateable,
};

impl JsTypeGenerateable for OptionalMeta {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens {
        let type_accessor = self.inner.gen_ser_accessor(variable_path.to_owned());
        quote!(if ($variable_path !== undefined) { s.serialize_number(U32_BYTES, false, 1); $type_accessor } else { s.serialize_number(U32_BYTES, false, 0) })
    }

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens {
        let inner_accessor = self.inner.gen_des_accessor(FieldAccessor::None);
        quote!($(field_accessor)(d.deserialize_number(U32_BYTES, false) === 0) ? undefined : $inner_accessor)
    }

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens {
        let available_check =
            ty_check::AvailableCheck::from_variable_path(variable_path.to_owned());
        let inner_type_check = self.inner.gen_ty_check(variable_path.to_owned());
        match &available_check {
            AvailableCheck::Object(_, _) => {
                quote!((($(available_check.to_owned()) && ($(variable_path.to_owned()) !== undefined && $inner_type_check) || $variable_path === undefined) || !($available_check)))
            }
            AvailableCheck::None => {
                quote!(($(variable_path.to_owned()) !== undefined && $inner_type_check) || $variable_path === undefined)
            }
        }
    }

    fn gen_ts_type(&self) -> Tokens {
        quote!($(self.inner.gen_ts_type()) | undefined)
    }
}
