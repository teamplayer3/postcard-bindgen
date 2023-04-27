pub mod ts;

use genco::{lang::js::Tokens, quote};

use crate::{code_gen::utils::line_brake_chain, registry::BindingType, utils::StrExt};

use super::{generateable::container::BindingTypeGenerateable, JS_OBJECT_VARIABLE};

pub fn gen_type_checkings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    line_brake_chain(bindings.as_ref().iter().map(gen_type_check))
}

pub fn gen_type_check(binding_type: &BindingType) -> Tokens {
    let type_name = binding_type.inner_name().to_obj_identifier();
    let body = binding_type.gen_ty_check_body();
    quote!(const is_$type_name = ($JS_OBJECT_VARIABLE) => ($body))
}
