pub mod impls;

use genco::prelude::js::Tokens;

pub trait AccessorGenerateable {
    fn gen_ser_accessor(&self, variable_path: ser::VariablePath) -> Tokens;

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens;

    fn gen_ty_check(&self, variable_path: ser::VariablePath) -> Tokens;
}

pub mod ser {
    use genco::{prelude::JavaScript, quote_in, tokens::FormatInto};

    use crate::code_gen::JS_OBJECT_VARIABLE;

    // v .name
    // v [0]
    // v .name .hello
    // v [1] .hello
    // v .name [0]
    // v .hello .name

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct VariablePath {
        parts: Vec<VariableAccess>,
        start_variable: String,
    }

    impl Default for VariablePath {
        fn default() -> Self {
            Self {
                parts: Default::default(),
                start_variable: JS_OBJECT_VARIABLE.into(),
            }
        }
    }

    impl VariablePath {
        pub fn new(start_variable: String) -> Self {
            Self {
                start_variable,
                parts: Default::default(),
            }
        }

        pub fn push(&mut self, part: VariableAccess) {
            self.parts.push(part)
        }

        pub fn modify_push(mut self, part: VariableAccess) -> Self {
            self.push(part);
            self
        }

        pub fn pop(&mut self) -> (&mut Self, Option<VariableAccess>) {
            let popped = self.parts.pop();
            (self, popped)
        }

        pub fn modify_pop(mut self) -> (Self, Option<VariableAccess>) {
            let (_, popped) = self.pop();
            (self, popped)
        }
    }

    impl FormatInto<JavaScript> for VariablePath {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(self.start_variable)
            }
            self.parts
                .into_iter()
                .for_each(|part| part.format_into(tokens))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum VariableAccess {
        Indexed(usize),
        Field(String),
    }

    impl FormatInto<JavaScript> for VariableAccess {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    Self::Indexed(index) => [$index],
                    Self::Field(name) => .$name,
                })
            }
        }
    }
}

pub mod des {
    use genco::{
        prelude::{js::Tokens, JavaScript},
        quote_in,
        tokens::FormatInto,
    };

    #[derive(Debug, Clone, Copy)]
    pub enum FieldAccessor<'a> {
        Object(&'a str),
        Array,
        None,
    }

    impl<'a> FormatInto<JavaScript> for FieldAccessor<'a> {
        fn format_into(self, tokens: &mut Tokens) {
            quote_in! { *tokens =>
                $(match self {
                    Self::Array | Self::None => (),
                    Self::Object(n) => $n:$[' '],
                })
            }
        }
    }
}

pub mod ty_check {
    use genco::{
        prelude::JavaScript,
        quote_in,
        tokens::{quoted, FormatInto},
    };

    use super::ser::{VariableAccess, VariablePath};

    #[derive(Clone)]
    pub enum AvailableCheck {
        Object(super::ser::VariablePath, String),
        None,
    }

    impl AvailableCheck {
        pub fn from_variable_path(path: VariablePath) -> Self {
            let (path, last) = path.modify_pop();
            match last {
                Some(VariableAccess::Field(name)) => Self::Object(path, name),
                _ => Self::None,
            }
        }
    }

    impl FormatInto<JavaScript> for AvailableCheck {
        fn format_into(self, tokens: &mut genco::Tokens<JavaScript>) {
            quote_in! { *tokens =>
                $(match self {
                    AvailableCheck::Object(path, name) => $(quoted(name)) in $path,
                    AvailableCheck::None => ()
                })
            }
        }
    }
}
