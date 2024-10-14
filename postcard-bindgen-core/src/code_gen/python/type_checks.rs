use genco::quote;

use crate::{
    code_gen::{
        python::{generateable::container::BindingTypeGenerateable, PYTHON_OBJECT_VARIABLE},
        utils::TokensIterExt,
    },
    registry::BindingType,
    utils::StrExt,
};

use super::Tokens;

pub fn gen_type_checkings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    bindings
        .as_ref()
        .iter()
        .map(gen_type_check)
        .join_with_empty_line()
}

fn gen_type_check(binding_type: &BindingType) -> Tokens {
    let type_name = binding_type.inner_name().to_obj_identifier();
    let body = binding_type.gen_ty_check_body();
    quote! {
        def assert_$type_name($PYTHON_OBJECT_VARIABLE):
            $body
    }
}
