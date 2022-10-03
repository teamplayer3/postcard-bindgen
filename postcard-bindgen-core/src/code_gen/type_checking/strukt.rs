use genco::{lang::js::Tokens, quote};

use crate::{registry::StructField, utils::StrExt};

use super::{gen_struct_field_available_checks, gen_struct_field_type_checks, InnerTypeAccess};

pub fn gen_check_func(obj_name: impl AsRef<str>, fields: impl AsRef<[StructField]>) -> Tokens {
    let obj_name = obj_name.as_ref();

    let field_available_checks =
        gen_struct_field_available_checks(&fields, InnerTypeAccess::Direct);
    let field_type_checks = gen_struct_field_type_checks(&fields, InnerTypeAccess::Direct);

    quote! {
        const is_$(obj_name.to_obj_identifier()) = (v) => {
            return typeof v === "object" && $field_available_checks && $field_type_checks
        }
    }
}
