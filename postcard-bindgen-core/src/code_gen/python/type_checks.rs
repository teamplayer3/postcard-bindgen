use genco::quote;

use crate::{
    code_gen::{
        python::{generateable::container::BindingTypeGenerateable, PYTHON_OBJECT_VARIABLE},
        utils::{StrExt, TokensIterExt},
    },
    registry::Container,
};

use super::Tokens;

pub fn gen_type_checkings(bindings: impl AsRef<[Container]>) -> Tokens {
    bindings
        .as_ref()
        .iter()
        .map(gen_type_check)
        .join_with_empty_line()
}

fn gen_type_check(container: &Container) -> Tokens {
    let type_name = container.name.to_obj_identifier();
    let body = container.r#type.gen_ty_check_body(container.name);
    quote! {
        def assert_$type_name($PYTHON_OBJECT_VARIABLE):
            $body
    }
}
