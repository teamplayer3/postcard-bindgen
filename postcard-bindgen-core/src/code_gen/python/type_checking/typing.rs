use genco::{lang::python::Tokens, quote};

use crate::registry::BindingType;


pub fn generate_typings(bindings: impl AsRef<[BindingType]>) -> Tokens {
    quote!()
}