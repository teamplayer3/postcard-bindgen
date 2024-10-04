pub mod array;
mod bool;
pub mod map;
pub mod number;
pub mod object;
pub mod optional;
pub mod range;
pub mod string;
pub mod tuple;

pub mod js_type;

use genco::prelude::js::Tokens;

use crate::code_gen::js::{FieldAccessor, VariablePath};

pub trait JsTypeGenerateable {
    fn gen_ser_accessor(&self, variable_path: VariablePath) -> Tokens;

    fn gen_des_accessor(&self, field_accessor: FieldAccessor) -> Tokens;

    fn gen_ty_check(&self, variable_path: VariablePath) -> Tokens;

    fn gen_ts_type(&self) -> Tokens;
}

pub mod ty_check {
    use genco::{
        prelude::JavaScript,
        quote_in,
        tokens::{quoted, FormatInto},
    };

    use crate::code_gen::js::{VariableAccess, VariablePath};

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
