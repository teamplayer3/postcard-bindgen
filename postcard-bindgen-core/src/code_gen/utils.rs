use core::ops::Deref;

use convert_case::{Case, Casing};
use genco::{
    lang::Lang,
    quote,
    tokens::{FormatInto, Item},
    Tokens,
};

use crate::{
    path::PathBuf,
    registry::{Container, ContainerInfo},
    type_info::ObjectMeta,
};

pub fn break_long_logical_lines<L: Lang>(tokens: impl FormatInto<L>) -> Tokens<L> {
    let tokens = quote!($tokens);
    let mut result = Tokens::new();
    let mut intend = false;

    for token in tokens {
        let is_logical_operator =
            matches!(&token, Item::Literal(l) if ["&&", "||"].contains(&l.deref()));
        result.append(token);
        if is_logical_operator {
            result.push();
            if !intend {
                result.indent();
                intend = true;
            }
        }
    }

    result.unindent();
    result
}

pub trait StrExt {
    fn to_obj_identifier(&self) -> String;
}

impl StrExt for &str {
    fn to_obj_identifier(&self) -> String {
        self.to_case(Case::Snake).to_uppercase()
    }
}

pub fn wrapped_brackets<L: Lang>(inner: Tokens<L>) -> Tokens<L> {
    quote!(($inner))
}

pub fn wrapped_curly_brackets<L: Lang>(inner: Tokens<L>) -> Tokens<L> {
    quote!({ $inner })
}

pub(super) trait TokensIterExt<L: Lang>: Iterator<Item = Tokens<L>>
where
    Self: Sized,
{
    const LOGICAL_OR: &'static str;
    const LOGICAL_AND: &'static str;

    fn join_with_line_breaks(mut self) -> Tokens<L> {
        let init = quote!($(self.next().unwrap()));
        self.fold(init, |mut acc, x| {
            acc.push();
            acc.append(x);
            acc
        })
    }

    fn join_with_empty_line(mut self) -> Tokens<L> {
        let init = quote!($(self.next().unwrap()));
        self.fold(init, |mut acc, x| {
            acc.line();
            acc.append(x);
            acc
        })
    }

    fn join_with_comma(self) -> Tokens<L> {
        quote!($(for part in self join (, ) => $part))
    }

    fn join_with_comma_min_one(self) -> Tokens<L> {
        let mut parts = self.peekable();
        let mut init = quote!($(parts.next().unwrap()));

        if parts.peek().is_none() {
            init.append(",");
            return init;
        }

        parts.fold(init, |mut acc, x| {
            acc.append(",");
            acc.space();
            acc.append(x);
            acc
        })
    }

    fn join_with_semicolon(self) -> Tokens<L> {
        quote!($(for part in self join (; ) => $part))
    }

    fn join_with_colon(self) -> Tokens<L> {
        quote!($(for part in self join ( : ) => $part))
    }

    fn join_with_vertical_line(self) -> Tokens<L> {
        quote!($(for part in self join ( | ) => $part))
    }

    fn join_logic_and(self) -> Tokens<L> {
        quote!($(for part in self join ( $(Self::LOGICAL_AND) ) => $part))
    }

    fn join_logic_or(self) -> Tokens<L> {
        quote!($(for part in self join ( $(Self::LOGICAL_OR) ) => $part))
    }
}

pub(super) trait IfBranchedTemplate<L: Lang> {
    const IF_BRANCH: &'static str;
    const IF_ELSE_BRANCH: &'static str;
    const ELSE_BRANCH: &'static str;

    fn push_condition(tokens: &mut Tokens<L>, condition: impl FormatInto<L>);

    fn push_condition_block(tokens: &mut Tokens<L>, body: impl FormatInto<L>);
}

pub(super) trait TokensBranchedIterExt<L: Lang>:
    Iterator<Item = (Option<Tokens<L>>, Tokens<L>)>
where
    Self: Sized,
{
    type Template: IfBranchedTemplate<L>;

    fn join_if_branched(self) -> Tokens<L> {
        let mut tokens = Tokens::new();
        let mut next_items = self.peekable();

        let mut is_first = true;
        while let Some((condition, body)) = next_items.next() {
            let is_last = next_items.peek().is_none();

            let branch_statement = if is_first {
                is_first = false;
                Self::Template::IF_BRANCH
            } else if is_last {
                Self::Template::ELSE_BRANCH
            } else {
                Self::Template::IF_ELSE_BRANCH
            };

            tokens.append(branch_statement);

            if !is_last {
                tokens.space();
                Self::Template::push_condition(
                    &mut tokens,
                    condition.expect("if branch needs condition"),
                );
            }
            Self::Template::push_condition_block(&mut tokens, body);
        }

        tokens
    }
}

pub struct ContainerIdentifierBuilder<'a> {
    path: PathBuf<'a>,
    name: &'a str,
}

impl<'a> ContainerIdentifierBuilder<'a> {
    pub fn new(path: PathBuf<'a>, name: &'a str) -> Self {
        Self { path, name }
    }

    pub fn build(mut self) -> String {
        // We will skip the first part of the path, as it is the crate name.
        self.path.pop_front();
        self.path.push(self.name.to_obj_identifier());

        self.path.into_path("_").to_string()
    }
}

impl From<&Container> for ContainerIdentifierBuilder<'_> {
    fn from(container: &Container) -> Self {
        Self::new(container.path.clone().into_buf(), container.name)
    }
}

impl From<&ObjectMeta> for ContainerIdentifierBuilder<'_> {
    fn from(meta: &ObjectMeta) -> Self {
        Self::new(meta.path.clone().into_buf(), meta.name)
    }
}

pub struct ContainerFullQualifiedTypeBuilder<'a> {
    path: PathBuf<'a>,
    name: &'a str,
}

impl ContainerFullQualifiedTypeBuilder<'_> {
    pub fn new<'a>(path: PathBuf<'a>, name: &'a str) -> ContainerFullQualifiedTypeBuilder<'a> {
        ContainerFullQualifiedTypeBuilder { path, name }
    }

    pub fn build(mut self) -> String {
        // We will skip the first part of the path, as it is the crate name.
        self.path.pop_front();
        self.path.push(self.name);

        self.path.into_path(".").to_string()
    }
}

impl<'a> From<&'a ContainerInfo<'a>> for ContainerFullQualifiedTypeBuilder<'a> {
    fn from(container: &'a ContainerInfo<'a>) -> Self {
        Self::new(container.path.clone().into_buf(), container.name.as_ref())
    }
}

impl From<&Container> for ContainerFullQualifiedTypeBuilder<'_> {
    fn from(container: &Container) -> Self {
        Self::new(container.path.clone().into_buf(), container.name)
    }
}

impl From<&ObjectMeta> for ContainerFullQualifiedTypeBuilder<'_> {
    fn from(meta: &ObjectMeta) -> Self {
        Self::new(meta.path.clone().into_buf(), meta.name)
    }
}

#[cfg(test)]
pub fn assert_tokens(generated: genco::lang::js::Tokens, compare: genco::lang::js::Tokens) {
    assert_eq!(generated.to_file_string(), compare.to_file_string())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_container_identifier_builder() {
        let container = ContainerInfo {
            name: "Test".into(),
            path: PathBuf::new().into_path("::"),
        };

        let builder =
            ContainerIdentifierBuilder::new(container.path.into_buf(), container.name.as_ref());

        assert_eq!(builder.build(), "TEST".to_string());

        let container = ContainerInfo {
            name: "Test".into(),
            path: PathBuf::from_iter(["crate".into(), "submodule".into()]).into_path("::"),
        };

        let builder =
            ContainerIdentifierBuilder::new(container.path.into_buf(), container.name.as_ref());

        assert_eq!(builder.build(), "submodule_TEST".to_string());
    }

    #[test]
    fn test_container_full_qualified_type_builder() {
        let container = ContainerInfo {
            name: "Test".into(),
            path: PathBuf::new().into_path("::"),
        };

        let builder: ContainerFullQualifiedTypeBuilder = (&container).into();

        assert_eq!(builder.build(), "Test".to_string());

        let container = ContainerInfo {
            name: "Test".into(),
            path: PathBuf::from_iter(["crate".into(), "submodule".into()]).into_path("::"),
        };

        let builder: ContainerFullQualifiedTypeBuilder = (&container).into();

        assert_eq!(builder.build(), "submodule.Test".to_string());
    }
}
