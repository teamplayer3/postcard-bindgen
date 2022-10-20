use genco::{prelude::JavaScript, quote_in, tokens::FormatInto};

use super::JS_OBJECT_VARIABLE;

pub mod binding_tys;
pub mod js_types;

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
