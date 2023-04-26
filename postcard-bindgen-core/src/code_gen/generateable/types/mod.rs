pub mod array;
pub mod map;
pub mod number;
pub mod object;
pub mod optional;
pub mod range;
pub mod string;

pub mod js_type;

use genco::prelude::js::Tokens;

use super::VariablePath;

pub trait JsTypeGenerateable {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens;

    fn gen_des_accessor(&self, field_accessor: des::FieldAccessor) -> Tokens;

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens;

    fn gen_ts_type(&self) -> Tokens;
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

    use crate::code_gen::generateable::{VariableAccess, VariablePath};

    #[derive(Clone)]
    pub enum AvailableCheck {
        Object(VariablePath, String),
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
