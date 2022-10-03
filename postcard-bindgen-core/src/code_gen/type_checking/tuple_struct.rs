use genco::{lang::js::Tokens, quote};

use crate::{type_info::JsType, StrExt};

use super::{gen_array_field_type_checks, InnerTypeAccess};

pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[JsType]>) -> Tokens {
    let obj_name = obj_name.as_ref();

    let field_count = fields.as_ref().len();
    let field_type_checks = gen_array_field_type_checks(fields, InnerTypeAccess::Direct);

    quote! {
        const is_$(obj_name.to_obj_identifier()) = (v) => {
            return Array.isArray(v) && v.length === $field_count && $field_type_checks
        }
    }
}
