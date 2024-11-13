use genco::quote;

use crate::{
    code_gen::{
        python::{generateable::container::BindingTypeGenerateable, PYTHON_OBJECT_VARIABLE},
        utils::{ContainerIdentifierBuilder, TokensIterExt},
    },
    registry::Container,
};

use super::Tokens;

pub fn gen_type_checks(bindings: impl Iterator<Item = Container>) -> Tokens {
    bindings.map(gen_type_check).join_with_empty_line()
}

fn gen_type_check(container: Container) -> Tokens {
    let container_ident = ContainerIdentifierBuilder::new(&container.path, container.name).build();
    let body = container
        .r#type
        .gen_ty_check_body(container.name, container.path);
    quote! {
        def assert_$container_ident($PYTHON_OBJECT_VARIABLE):
            $body
    }
}
