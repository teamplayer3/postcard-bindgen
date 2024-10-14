use std::collections::HashMap;

use genco::{lang::Lang, Tokens};

#[derive(Debug, Clone)]
pub enum ImportItem<L: Lang> {
    All,
    Single(Tokens<L>),
}

#[derive(Debug)]
pub enum ImportMode<L: Lang> {
    All,
    Single(Vec<Tokens<L>>),
}

pub struct ImportRegistry<L: Lang> {
    imports: HashMap<Tokens<L>, ImportMode<L>>,
}

impl<L: Lang> ImportRegistry<L> {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
        }
    }

    pub fn push(&mut self, package: Tokens<L>, item: ImportItem<L>) {
        self.imports
            .entry(package)
            .and_modify(|e| match &item {
                ImportItem::All => *e = ImportMode::All,
                ImportItem::Single(item) => match e {
                    ImportMode::All => (),
                    ImportMode::Single(imports) => imports.push(item.to_owned()),
                },
            })
            .or_insert_with(|| match item {
                ImportItem::All => ImportMode::All,
                ImportItem::Single(item) => ImportMode::Single(vec![item]),
            });
    }

    pub(super) fn into_items_sorted(self) -> Vec<(Tokens<L>, ImportMode<L>)> {
        let mut items = self.imports.into_iter().collect::<Vec<_>>();
        items.sort_by_key(|i| i.0.to_owned());

        items
    }
}
