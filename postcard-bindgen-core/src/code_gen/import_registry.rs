use std::{borrow::Cow, collections::HashMap};

use crate::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ImportItem {
    All,
    Single(Cow<'static, str>),
    Aliased {
        item_name: Cow<'static, str>,
        alias: Cow<'static, str>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub name: Cow<'static, str>,
    pub alias: Option<Cow<'static, str>>,
}

impl Import {
    fn new(name: Cow<'static, str>) -> Self {
        Self { name, alias: None }
    }

    fn aliased(name: Cow<'static, str>, alias: Cow<'static, str>) -> Self {
        Self {
            name,
            alias: Some(alias),
        }
    }
}

impl PartialOrd for Import {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Import {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportMode {
    All,
    Single(Vec<Import>),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Package {
    Extern(PathBuf<'static>),
    Intern(PathBuf<'static>),
    Relative(PathBuf<'static>),
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Extern(name) => match other {
                Self::Extern(other_name) => name.cmp(other_name),
                _ => std::cmp::Ordering::Less,
            },
            Self::Intern(name) => match other {
                Self::Extern(_) => std::cmp::Ordering::Greater,
                Self::Intern(other_name) => name.cmp(other_name),
                Self::Relative(_) => std::cmp::Ordering::Less,
            },
            Self::Relative(name) => match other {
                Self::Relative(other_name) => name.cmp(other_name),
                _ => std::cmp::Ordering::Greater,
            },
        }
    }
}

pub struct ImportRegistry {
    imports: HashMap<Package, ImportMode>,
    base_path: String,
}

impl ImportRegistry {
    pub fn new(base_path: String) -> Self {
        Self {
            imports: HashMap::new(),
            base_path,
        }
    }

    pub fn push(&mut self, package: Package, item: ImportItem) {
        self.imports
            .entry(package)
            .and_modify(|e| match &item {
                ImportItem::All => *e = ImportMode::All,
                ImportItem::Single(item) => match e {
                    ImportMode::All => (),
                    ImportMode::Single(imports) => {
                        let import = Import::new(item.clone());
                        if !imports.contains(&import) {
                            imports.push(import);
                            imports.sort();
                        }
                    }
                },
                ImportItem::Aliased { item_name, alias } => match e {
                    ImportMode::All => (),
                    ImportMode::Single(imports) => {
                        let import = Import::aliased(item_name.clone(), alias.clone());
                        if !imports.contains(&import) {
                            imports.push(import);
                            imports.sort();
                        }
                    }
                },
            })
            .or_insert_with(|| match item {
                ImportItem::All => ImportMode::All,
                ImportItem::Single(item) => ImportMode::Single(vec![Import::new(item)]),
                ImportItem::Aliased { item_name, alias } => {
                    ImportMode::Single(vec![Import::aliased(item_name, alias)])
                }
            });
    }

    pub(super) fn into_items_sorted(self) -> (String, Vec<(Package, ImportMode)>) {
        let mut items = self.imports.into_iter().collect::<Vec<_>>();
        items.sort_by_key(|(package, ..)| package.to_owned());

        (self.base_path, items)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_items_sorted() {
        let mut registry = ImportRegistry::new("base_package".into());

        registry.push(
            Package::Relative("rel".into()),
            ImportItem::Single("test".into()),
        );
        registry.push(
            Package::Relative("rel".into()),
            ImportItem::Single("a_test".into()),
        );
        registry.push(
            Package::Extern("extern".into()),
            ImportItem::Single("test".into()),
        );

        registry.push(Package::Intern("package".into()), ImportItem::All);

        let (base_path, items) = registry.into_items_sorted();

        assert_eq!(base_path, "base_package");

        assert_eq!(
            items,
            vec![
                (
                    Package::Extern("extern".into()),
                    ImportMode::Single(vec![Import::new("test".into())])
                ),
                (Package::Intern("package".into()), ImportMode::All),
                (
                    Package::Relative("rel".into()),
                    ImportMode::Single(vec![
                        Import::new("a_test".into()),
                        Import::new("test".into())
                    ])
                ),
            ]
        );
    }
}
