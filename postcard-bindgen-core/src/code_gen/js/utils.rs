use genco::{quote, tokens::quoted};

use crate::utils::ContainerPath;

use super::Tokens;

pub struct ContainerCaseTypeBuilder<'a> {
    path: &'a ContainerPath<'a>,
    name: &'a str,
}

impl ContainerCaseTypeBuilder<'_> {
    pub fn new<'a>(path: &'a ContainerPath<'a>, name: &'a str) -> ContainerCaseTypeBuilder<'a> {
        ContainerCaseTypeBuilder { path, name }
    }

    pub fn build(&self) -> Tokens {
        let chained_parts: Vec<_> = self
            .path
            .parts()
            .skip(1)
            .chain(std::iter::once(self.name))
            .collect();
        let joined = chained_parts.join(".");
        quote!($(quoted(joined)))
    }
}
