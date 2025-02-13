mod des;
mod general;
mod generateable;
mod ser;
mod type_checks;

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
use type_checks::gen_type_checks;

use crate::{registry::ContainerCollection, ExportFile, Exports};

use super::{export_registry::ExportMode, utils::TokensIterExt};

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
type Function = super::function::Function<JavaScript>;
type FunctionArg = super::function::FunctionArg<JavaScript>;
type ExportRegistry = super::export_registry::ExportRegistry<JavaScript>;
type Case = super::switch_case::Case<JavaScript>;
type DefaultCase = super::switch_case::DefaultCase<JavaScript>;
type SwitchCase = super::switch_case::SwitchCase<JavaScript>;

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
    module_structure: bool,
    esm_module: bool,
}

impl GenerationSettings {
    /// Constructs [`GenerationSettings`] and enables all options at once.
    pub fn enable_all() -> Self {
        Self {
            ser: true,
            des: true,
            runtime_type_checks: true,
            type_script_types: true,
            module_structure: true,
            esm_module: true,
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

    /// Enabling or disabling of module structure code generation.
    ///
    /// Enabling this will generate the types in typescript in the same module structure
    /// as in rust. Root level types will be in the root of the generated
    /// package. Types nested in modules will be in namespaces
    /// (e.g. <mod_name>.<type_name>). This avoids name clashes.
    ///
    /// Disabling this will generate all types in the root module.
    pub fn module_structure(mut self, enabled: bool) -> Self {
        self.module_structure = enabled;
        self
    }

    /// Enabling or disabling ESM (as opposed to cjs) output
    ///
    /// Enabling will change the way the `serialize` and `desrialize`
    /// functions are exported to bring them in line with ESM standards/importers.
    /// The package.json file also gets `"type": "module"` added,
    /// so package managers/bundlers importing it know it's ESM.
    ///
    ///
    /// Disabling this will use the default `module.exports`-style export (cjs)
    pub fn esm_module(mut self, enabled: bool) -> Self {
        self.esm_module = enabled;
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
            module_structure: true,
            esm_module: false,
        }
    }
}

/// Metadata for JS export
///
/// Contains information about the exported JS package needed to
/// complete the full npm_package (e.g. if it's an ESM module or not)
pub struct ExportMeta {
    pub esm_module: bool,
}

pub fn generate(
    mut containers: ContainerCollection,
    gen_settings: impl Borrow<GenerationSettings>,
) -> (Exports<JavaScript>, ExportMeta) {
    let gen_settings = gen_settings.borrow();

    if !gen_settings.module_structure {
        containers.flatten();
    }

    let export_mode = if gen_settings.esm_module {
        ExportMode::Esm
    } else {
        ExportMode::Cjs
    };

    let mut export_files = Vec::new();

    export_files.push(ExportFile {
        content_type: "util".to_owned(),
        content: gen_util(),
    });

    if gen_settings.ser {
        export_files.push(ExportFile {
            content_type: "serializer".to_owned(),
            content: gen_serializer_code(),
        });

        let mut tokens = Tokens::new();

        tokens.append(gen_ser_functions(containers.all_containers()));
        tokens.line();

        let mut export_registry = ExportRegistry::new(export_mode.clone());

        tokens.append(gen_serialize_func(
            containers.all_containers(),
            gen_settings.runtime_type_checks,
            &mut export_registry,
        ));

        tokens.line();
        tokens.append(export_registry);

        export_files.push(ExportFile {
            content_type: "ser".to_owned(),
            content: tokens,
        });
    }

    if gen_settings.des {
        export_files.push(ExportFile {
            content_type: "deserializer".to_owned(),
            content: gen_deserializer_code(),
        });

        let mut tokens = Tokens::new();

        tokens.append(gen_des_functions(containers.all_containers()));
        tokens.line();

        let mut export_registry = ExportRegistry::new(export_mode);

        tokens.append(gen_deserialize_func(
            containers.all_containers(),
            &mut export_registry,
        ));
        tokens.line();

        tokens.append(export_registry);

        export_files.push(ExportFile {
            content_type: "des".to_owned(),
            content: tokens,
        });
    }

    if gen_settings.runtime_type_checks {
        export_files.push(ExportFile {
            content_type: "runtime_checks".to_owned(),
            content: gen_type_checks(containers.all_containers()),
        });
    }

    if gen_settings.type_script_types {
        let ts = gen_ts_typings(&containers, gen_settings);
        export_files.push(ExportFile {
            content_type: "ts".to_owned(),
            content: ts,
        });
    }

    // Create metadata about export
    let export_metadata = ExportMeta {
        esm_module: gen_settings.esm_module,
    };

    (
        Exports {
            files: export_files,
        },
        export_metadata,
    )
}

impl<I, F> TokensIterExt<JavaScript, F> for I
where
    I: Iterator<Item = F>,
    F: FormatInto<JavaScript>,
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

impl FormatInto<JavaScript> for FunctionArg {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            $(self.name)
        }
    }
}

impl FormatInto<JavaScript> for Function {
    fn format_into(self, tokens: &mut Tokens) {
        let doc_string = self.doc_string.map(|doc| {
            let mut tokens = Tokens::new();
            tokens.append("/**\n");
            tokens.append(
                doc.lines()
                    .map(|line| format!(" * {}", line.trim()))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            tokens.push();
            tokens.append(" */");
            tokens
        });
        quote_in! { *tokens =>
            $(doc_string)
            function $(self.name)($(for arg in self.args join (, ) => $arg)) {
                $(self.body)
            }
        }
    }
}

impl FormatInto<JavaScript> for ExportRegistry {
    fn format_into(self, tokens: &mut Tokens) {
        match self.export_mode {
            ExportMode::Cjs => {
                quote_in! { *tokens =>
                    $(for export in self.exports join () => exports.$(&export) = $export)
                }
            }
            ExportMode::Esm => {
                quote_in! { *tokens =>
                    export {
                        $(for export in self.exports join (,) => $export)
                    };
                }
            }
        }
    }
}

impl FormatInto<JavaScript> for Case {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! {*tokens =>
            case $(self.case):
                $(self.body)
                $(if self.break_after { break; })
        }
    }
}

impl FormatInto<JavaScript> for DefaultCase {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            default:
                $(self.body)
                $(if self.break_after { break; })
        }
    }
}

impl FormatInto<JavaScript> for SwitchCase {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            switch ($(self.switch_arg)) {
            $(for case in self.cases => $case)
            $(if let Some(default_case) = self.default_case { $default_case })
            }
        }
    }
}
