use container::BindingTypeGenerateable;
use genco::quote;

use crate::{
    code_gen::{python::ImportRegistry, utils::TokensIterExt},
    registry::Container,
};

use super::Tokens;

pub mod container;
pub mod types;

pub fn gen_typings(binding_type: impl AsRef<[Container]>) -> Tokens {
    let mut import_registry = ImportRegistry::new();
    let typings = binding_type
        .as_ref()
        .iter()
        .map(|t| t.r#type.gen_typings_body(t.name, &mut import_registry))
        .join_with_line_breaks();

    quote! {
        $import_registry

        u8 = int
        i8 = int
        u16 = int
        i16 = int
        u32 = int
        i32 = int
        u64 = int
        i64 = int

        $typings
    }
}
