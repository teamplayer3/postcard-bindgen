use genco::{lang::Lang, quote, tokens::FormatInto, Tokens};

#[derive(Debug, Default, Clone)]
pub enum ExportMode {
    #[default]
    Cjs,
    Esm,
}

pub struct ExportRegistry<L>
where
    L: Lang,
{
    pub(super) exports: Vec<Tokens<L>>,
    pub(super) export_mode: ExportMode,
}

impl<L> ExportRegistry<L>
where
    L: Lang,
{
    pub fn new(export_mode: ExportMode) -> Self {
        Self {
            exports: Vec::new(),
            export_mode,
        }
    }
}

impl<L> ExportRegistry<L>
where
    L: Lang,
{
    pub fn push(&mut self, export: impl FormatInto<L>) {
        self.exports.push(quote!($export));
    }
}
