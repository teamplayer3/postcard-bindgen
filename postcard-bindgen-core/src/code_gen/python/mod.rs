mod des;
mod general;
mod generateable;
mod ser;
mod type_checks;

use core::borrow::Borrow;

use convert_case::{Case, Casing};
use des::{gen_des_functions, gen_deserialize_func, gen_deserializer_code};
use genco::{lang::python::Python, quote, quote_in, tokens::FormatInto};
use general::gen_util;
use generateable::{gen_basic_typings, gen_typings};
use ser::{gen_ser_functions, gen_serialize_func, gen_serializer_code};
use type_checks::gen_type_checks;

use crate::{code_gen::import_registry::ImportMode, registry::ContainerCollection, Exports};

use super::{
    import_registry::{ImportItem, Package},
    utils::{IfBranchedTemplate, TokensBranchedIterExt, TokensIterExt},
};

const PYTHON_OBJECT_VARIABLE: &str = "v";
const PYTHON_LOGIC_AND: &str = "and";
const PYTHON_LOGIC_OR: &str = "or";

type Tokens = genco::lang::python::Tokens;

type VariablePath = super::variable_path::VariablePath<Python>;
type VariableAccess = super::variable_path::VariableAccess;
type FieldAccessor<'a> = super::field_accessor::FieldAccessor<'a>;
type AvailableCheck = super::available_check::AvailableCheck<Python>;
type ImportRegistry = super::import_registry::ImportRegistry;
type ExportFile = crate::ExportFile<Python>;

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
}

impl GenerationSettings {
    /// Constructs [`GenerationSettings`] and enables all options at once.
    pub fn enable_all() -> Self {
        Self {
            ser: true,
            des: true,
            runtime_type_checks: true,
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
        }
    }
}

pub fn generate(
    containers: &ContainerCollection,
    gen_settings: impl Borrow<GenerationSettings>,
    generate_package_name: String,
) -> Exports<Python> {
    let generate_package_name = generate_package_name.to_case(Case::Snake);
    let gen_settings = gen_settings.borrow();
    let mut files = Vec::new();

    files.push(ExportFile {
        content_type: "util".to_owned(),
        content: gen_util(),
    });

    files.push(ExportFile {
        content_type: "basic_types".to_owned(),
        content: gen_basic_typings(),
    });

    files.extend(gen_typings(containers, generate_package_name.clone()));

    if gen_settings.runtime_type_checks {
        let type_checks = gen_type_checks(containers.all_containers());

        let type_checks = quote! {
            from .util import *
            from .types import *

            $type_checks
        };

        files.push(ExportFile {
            content_type: "type_checks".to_owned(),
            content: type_checks,
        });
    }

    if gen_settings.ser {
        let serializer_code = gen_serializer_code();
        let ser_code = quote! {
            from typing import Union

            from .types import *
            from .util import *
            from .serializer import Serializer

            $(gen_ser_functions(containers.all_containers()))

            $(gen_serialize_func(containers.all_containers(), gen_settings.runtime_type_checks))
        };

        files.push(ExportFile {
            content_type: "serializer".to_owned(),
            content: serializer_code,
        });

        files.push(ExportFile {
            content_type: "ser".to_owned(),
            content: ser_code,
        });
    }

    if gen_settings.des {
        let deserializer_code = gen_deserializer_code();
        let des_code = quote! {
            from typing import TypeVar, Type, cast

            from .types import *
            from .util import *
            from .deserializer import Deserializer

            $(gen_des_functions(containers.all_containers()))

            $(gen_deserialize_func(containers.all_containers()))
        };

        files.push(ExportFile {
            content_type: "deserializer".to_owned(),
            content: deserializer_code,
        });

        files.push(ExportFile {
            content_type: "des".to_owned(),
            content: des_code,
        });
    }

    let mut import_registry = ImportRegistry::new(generate_package_name);
    import_registry.push(Package::Relative("types".into()), ImportItem::All);
    import_registry.push(Package::Relative("basic_types".into()), ImportItem::All);

    if gen_settings.des {
        import_registry.push(
            Package::Relative("des".into()),
            ImportItem::Single("deserialize".into()),
        );
    }

    if gen_settings.ser {
        import_registry.push(
            Package::Relative("ser".into()),
            ImportItem::Single("serialize".into()),
        );
    }

    files.push(ExportFile {
        content_type: "__init__".to_owned(),
        content: quote!($import_registry),
    });

    Exports { files }
}

impl<I> TokensIterExt<Python> for I
where
    I: Iterator<Item = Tokens>,
{
    const LOGICAL_AND: &'static str = PYTHON_LOGIC_AND;
    const LOGICAL_OR: &'static str = PYTHON_LOGIC_OR;
}

pub(super) struct BranchedTemplate;

impl IfBranchedTemplate<Python> for BranchedTemplate {
    const IF_BRANCH: &'static str = "if";
    const IF_ELSE_BRANCH: &'static str = "elif";
    const ELSE_BRANCH: &'static str = "else";

    fn push_condition(tokens: &mut Tokens, condition: impl FormatInto<Python>) {
        tokens.append(condition)
    }

    fn push_condition_block(tokens: &mut Tokens, body: impl FormatInto<Python>) {
        tokens.append(":");
        tokens.indent();
        tokens.append(body);
        tokens.unindent();
    }
}

impl<I> TokensBranchedIterExt<Python> for I
where
    I: Iterator<Item = (Option<Tokens>, Tokens)>,
{
    type Template = BranchedTemplate;
}

impl FormatInto<Python> for FieldAccessor<'_> {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            $(match self {
                Self::Array | Self::None => (),
                Self::Object(n) => $n = $[' '],
            })
        }
    }
}

impl FormatInto<Python> for VariablePath {
    fn format_into(self, tokens: &mut genco::Tokens<Python>) {
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
        Self::new(PYTHON_OBJECT_VARIABLE.to_owned())
    }
}

impl FormatInto<Python> for VariableAccess {
    fn format_into(self, tokens: &mut genco::Tokens<Python>) {
        quote_in! { *tokens =>
            $(match self {
                Self::Indexed(index) => [$index],
                Self::Field(name) => .$name,
            })
        }
    }
}

impl FormatInto<Python> for AvailableCheck {
    fn format_into(self, tokens: &mut Tokens) {
        quote_in! { *tokens =>
            $(match self {
                AvailableCheck::Object(..) => (),
                AvailableCheck::None => ()
            })
        }
    }
}

impl FormatInto<Python> for ImportRegistry {
    fn format_into(self, tokens: &mut Tokens) {
        let (base_path, items) = self.into_items_sorted();
        for (package, imports) in items {
let joiner = ".";
            let package = match package {
                Package::Relative(path) => format!(".{}", path.into_path(joiner).to_string()),
                Package::Extern(path) => path.into_path(joiner).to_string(),
                Package::Intern(mut path) => {
                    if !path.is_empty() {
                        path.push_front(base_path.as_str());
path.into_path(joiner).to_string()
                    } else {
PathBuf::new()
                            .join(base_path.as_str())
.into_path(joiner)
                            .to_string()
                    }
                }
            };

            quote_in!(*tokens=> from $(package) import);
            tokens.space();

            match imports {
                ImportMode::All => quote_in!(*tokens=> *),
                ImportMode::Single(items) => {
                    let items = items.iter().map(|i| {
                        if let Some(alias) = &i.alias {
                            quote!($(&i.name) as $alias)
                        } else {
                            quote!($(&i.name))
                        }
                    });
                    quote_in!(*tokens=> $(for part in items join (, ) => $part))
                }
            }

            tokens.push();
        }
    }
}

#[cfg(test)]
mod test {
    use genco::tokens::FormatInto;

    use super::Tokens;

    #[test]
    fn test_import_registry_format() {
        use super::{ImportItem, ImportRegistry, Package};

        let mut import_registry = ImportRegistry::new("package".to_owned());
        import_registry.push(
            Package::Relative("basic_types".into()),
            ImportItem::Aliased {
                item_name: "A".into(),
                alias: "A__A".into(),
            },
        );
        import_registry.push(
            Package::Intern("des".into()),
            ImportItem::Single("deserialize".into()),
        );
        import_registry.push(
            Package::Extern("ser".into()),
            ImportItem::Single("serialize".into()),
        );
        import_registry.push(Package::Relative("types".into()), ImportItem::All);

        let mut tokens = Tokens::new();
        import_registry.format_into(&mut tokens);

        assert_eq!(
            tokens.to_file_string().unwrap(),
            format!(
                r#"from ser import serialize
from package.des import deserialize
from .basic_types import A as A__A
from .types import *
"#
            )
        );
    }
}
