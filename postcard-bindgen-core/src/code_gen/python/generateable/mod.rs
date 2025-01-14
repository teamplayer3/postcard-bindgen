use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use container::BindingTypeGenerateable;
use convert_case::{Case, Casing};
use genco::{quote, quote_in, tokens::quoted};

use crate::{
    code_gen::{python::ImportRegistry, utils::TokensIterExt},
    registry::{BindingType, Container, ContainerCollection, Module},
};

use super::{ExportFile, Tokens};

pub mod container;
pub mod types;

pub fn gen_typings(
    containers: &ContainerCollection,
    generate_package_name: String,
) -> Vec<ExportFile> {
    let mut files = Vec::new();

    let (containers, mods) = containers.containers_per_module();

    generate_typings_for_mod(
        "",
        containers.into_iter(),
        mods.into_iter(),
        &mut files,
        generate_package_name,
    );

    files
}

fn generate_typings_for_mod<'a>(
    package_path: impl AsRef<Path>,
    containers: impl Iterator<Item = Container> + Clone,
    mods: impl Iterator<Item = Module<'a>> + Clone,
    files: &mut Vec<ExportFile>,
    generate_package_name: String,
) {
    let container_exports = containers.clone().map(|f| {
        let mut l = vec![Cow::from(f.name)];
        if let BindingType::Enum(e) = f.r#type {
            l.extend(
                e.variants
                    .iter()
                    .map(|v| format!("{}_{}", f.name, v.name).into()),
            );
        }
        l
    });
    let mod_exports = mods.clone().map(|f| f.name().to_owned());

    let all_exports = container_exports
        .clone()
        .flatten()
        .chain(mod_exports.clone().map(|m| m.into()));

    let mut tokens = Tokens::new();

    let all_exports_tokens: Tokens = all_exports.map(|e| quote!($(quoted(e)))).join_with_comma();

    quote_in!(tokens=> __all__ = [
        $all_exports_tokens
    ]);

    tokens.line();

    for container_export in container_exports {
        let export_items = container_export.join(", ");
        quote_in!(tokens=> from ._$(container_export.first().unwrap().to_case(convert_case::Case::Snake)) import $export_items);
        tokens.push();
    }

    tokens.line();

    for mod_export in mod_exports {
        quote_in!(tokens=> from . import $mod_export);
        tokens.push();
    }

    let path = PathBuf::new().join("types").join(package_path);

    files.push(ExportFile {
        content_type: path.join("__init__").to_string_lossy().into_owned(),
        content: tokens,
    });

    for container in containers {
        let mut import_registry = ImportRegistry::new(generate_package_name.clone());
        let types = container
            .r#type
            .gen_typings_body((&container).into(), &mut import_registry);

        files.push(ExportFile {
            content_type: path
                .join(format!("_{}", container.name.to_case(Case::Snake)))
                .to_string_lossy()
                .into_owned(),
            content: quote! {
                $import_registry

                $types
            },
        });
    }

    for r#mod in mods {
        let (containers, mods) = r#mod.entries();

        generate_typings_for_mod(
            PathBuf::new().join(r#mod.path()).join(r#mod.name()),
            containers.into_iter(),
            mods.into_iter(),
            files,
            generate_package_name.clone(),
        );
    }
}

pub fn gen_basic_typings() -> Tokens {
    quote! {
        u8 = int
        i8 = int
        u16 = int
        i16 = int
        u32 = int
        i32 = int
        u64 = int
        i64 = int
        u128 = int
        i128 = int
    }
}
