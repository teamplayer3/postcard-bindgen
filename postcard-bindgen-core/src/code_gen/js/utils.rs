use genco::quote;

use crate::utils::ContainerPath;

use super::Tokens;

pub struct ContainerFullQualifiedTypeBuilder<'a> {
    path: &'a ContainerPath<'a>,
    name: &'a str,
}

impl ContainerFullQualifiedTypeBuilder<'_> {
    pub fn new<'a>(
        path: &'a ContainerPath<'a>,
        name: &'a str,
    ) -> ContainerFullQualifiedTypeBuilder<'a> {
        ContainerFullQualifiedTypeBuilder { path, name }
    }

    pub fn build(&self) -> Tokens {
        let chained_parts: Vec<_> = self
            .path
            .parts()
            .skip(1)
            .chain(std::iter::once(self.name))
            .collect();
        let joined = chained_parts.join(".");
        quote!($joined)
    }
}
