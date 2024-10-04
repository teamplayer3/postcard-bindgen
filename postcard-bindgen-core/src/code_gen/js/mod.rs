use genco::{
    prelude::js::{JavaScript, Tokens},
    quote, quote_in,
    tokens::FormatInto,
};

use crate::registry::BindingType;

use self::{
    ser_des::{
        gen_deserialize_func, gen_ser_des_classes, gen_ser_des_functions, gen_serialize_func,
    },
    type_checking::gen_type_checkings,
};

mod generateable;
pub mod ser_des;
pub mod type_checking;

const JS_ENUM_VARIANT_KEY: &str = "tag";
const JS_ENUM_VARIANT_VALUE: &str = "value";
const JS_OBJECT_VARIABLE: &str = "v";

type VariablePath = super::variable_path::VariablePath<JavaScript>;
type VariableAccess = super::variable_path::VariableAccess;
type FieldAccessor<'a> = super::field_accessor::FieldAccessor<'a>;

pub fn generate_js(tys: impl AsRef<[BindingType]>) -> Tokens {
    let ser_des_body = gen_ser_des_functions(&tys);
    let type_checks = gen_type_checkings(&tys);
    quote!(
        $(gen_ser_des_classes())
        $ser_des_body
        $type_checks
        $(gen_serialize_func(&tys))
        $(gen_deserialize_func(tys))
    )
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

impl Default for VariablePath {
    fn default() -> Self {
        Self::new(JS_OBJECT_VARIABLE.to_owned())
    }
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
