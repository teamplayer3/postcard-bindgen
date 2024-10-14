use genco::{lang::js::Tokens, quote};

use crate::{code_gen::utils::TokensIterExt, registry::BindingType, utils::StrExt};

use super::{generateable::container::BindingTypeGenerateable, JS_OBJECT_VARIABLE};

pub fn gen_type_checkings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    let body = bindings
        .as_ref()
        .iter()
        .map(gen_type_check)
        .join_with_line_breaks();
    quote! {
        const check_bounds = (n_bytes, signed, value) => { const max = BigInt(2 ** (n_bytes * BITS_PER_BYTE)), value_b = BigInt(value); if (signed) { const bounds = max / 2n; return value_b >= -bounds && value_b < bounds } else { return value_b < max && value_b >= 0 } }

        $body
    }
}

pub fn gen_type_check(binding_type: &BindingType) -> Tokens {
    let type_name = binding_type.inner_name().to_obj_identifier();
    let body = binding_type.gen_ty_check_body();
    quote!(const is_$type_name = ($JS_OBJECT_VARIABLE) => ($body))
}
