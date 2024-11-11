mod des;
mod general;
mod generateable;
mod ser;
mod type_checks;
mod utils;

use core::borrow::Borrow;

use des::{gen_des_functions, gen_deserialize_func, gen_deserializer_code};
use genco::{
    prelude::js::JavaScript,
    quote_in,
    tokens::{quoted, FormatInto},
};
use general::gen_util;
use generateable::gen_ts_typings;
use ser::{gen_ser_functions, gen_serialize_func, gen_serializer_code};
use type_checks::gen_type_checkings;

use crate::{registry::ContainerCollection, ExportFile, Exports};

use super::utils::TokensIterExt;

const JS_ENUM_VARIANT_KEY: &str = "tag";
const JS_ENUM_VARIANT_VALUE: &str = "value";
const JS_OBJECT_VARIABLE: &str = "v";
const JS_LOGIC_AND: &str = "&&";
const JS_LOGIC_OR: &str = "||";

type Tokens = genco::Tokens<JavaScript>;

type VariablePath = super::variable_path::VariablePath<JavaScript>;
type VariableAccess = super::variable_path::VariableAccess;
type FieldAccessor<'a> = super::field_accessor::FieldAccessor<'a>;
type AvailableCheck = super::available_check::AvailableCheck<JavaScript>;

/// Settings for bindings generation.
///
/// This enables the possibility to enable or disable serialization, deserialization, runtime type checks
/// or type script types.
/// Less code will be generated if an option is off.
///
/// By default, only deserialization is enabled. Serialization can be enabled by using [`GenerationSettings::serialization()`].
/// Deserialization can be disabled with [`GenerationSettings::deserialization()`].
/// To enable all at once use [`GenerationSettings::enable_all()`].
#[derive(Debug)]
pub struct GenerationSettings {
    ser: bool,
    des: bool,
    runtime_type_checks: bool,
    type_script_types: bool,
}

impl GenerationSettings {
    /// Constructs [`GenerationSettings`] and enables all options at once.
    pub fn enable_all() -> Self {
        Self {
            ser: true,
            des: true,
            runtime_type_checks: true,
            type_script_types: true,
        }
    }

    /// Enabling or disabling of serialization code generation.
    pub fn serialization(mut self, enabled: bool) -> Self {
        self.ser = enabled;
        self
    }

    /// Enabling or disabling of deserialization code generation.
    pub fn deserialization(mut self, enabled: bool) -> Self {
        self.des = enabled;
        self
    }

    /// Enabling or disabling of typescript types code generation.
    ///
    /// When enabling this, runtime type checks could be disabled with ['GenerationSettings::runtime_type_checks()']
    /// because the static type checks should enforce correct types. Additionally serialization should be faster
    /// without runtime type checks.
    pub fn type_script_types(mut self, enabled: bool) -> Self {
        self.type_script_types = enabled;
        self
    }

    /// Enabling or disabling of runtime type checks code generation.
    ///
    /// Disabling this should lead to a speed increase at serialization.
    pub fn runtime_type_checks(mut self, enabled: bool) -> Self {
        self.runtime_type_checks = enabled;
        self
    }
}

impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            ser: false,
            des: true,
            runtime_type_checks: false,
            type_script_types: false,
        }
    }
}

pub fn generate(
    containers: &ContainerCollection,
    gen_settings: impl Borrow<GenerationSettings>,
) -> Exports<JavaScript> {
    let gen_settings = gen_settings.borrow();

    let mut js_tokens = Tokens::new();

    js_tokens.append(gen_util());
    js_tokens.line();

    if gen_settings.ser {
        js_tokens.append(gen_serializer_code());
        js_tokens.line();
    }

    if gen_settings.des {
        js_tokens.append(gen_deserializer_code());
        js_tokens.line();
    }

    if gen_settings.ser {
        js_tokens.append(gen_ser_functions(containers.all_containers()));
        js_tokens.line();
    }

    if gen_settings.des {
        js_tokens.append(gen_des_functions(containers.all_containers()));
        js_tokens.line();
    }

    if gen_settings.runtime_type_checks {
        js_tokens.append(gen_type_checkings(containers.all_containers()));
        js_tokens.line();
    }

    if gen_settings.ser {
        js_tokens.append(gen_serialize_func(
            containers.all_containers(),
            gen_settings.runtime_type_checks,
        ));
        js_tokens.line();
    }

    if gen_settings.des {
        js_tokens.append(gen_deserialize_func(containers.all_containers()));
        js_tokens.line();
    }

    let mut export_files = vec![ExportFile {
        content_type: "js".to_owned(),
        content: js_tokens,
    }];

    if gen_settings.type_script_types {
        let ts = gen_ts_typings(containers);
        export_files.push(ExportFile {
            content_type: "ts".to_owned(),
            content: ts,
        });
    }

    Exports {
        files: export_files,
    }
}

impl<I> TokensIterExt<JavaScript> for I
where
    I: Iterator<Item = Tokens>,
{
    const LOGICAL_AND: &'static str = JS_LOGIC_AND;
    const LOGICAL_OR: &'static str = JS_LOGIC_OR;
}

impl FormatInto<JavaScript> for FieldAccessor<'_> {
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
    fn format_into(self, tokens: &mut Tokens) {
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
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            $(match self {
                Self::Indexed(index) => [$index],
                Self::Field(name) => .$name,
            })
        }
    }
}

impl FormatInto<JavaScript> for AvailableCheck {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            $(match self {
                AvailableCheck::Object(path, name) => $(quoted(name)) in $path,
                AvailableCheck::None => ()
            })
        }
    }
}
