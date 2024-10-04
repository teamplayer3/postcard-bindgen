use convert_case::{Case, Casing};

pub trait StrExt {
    fn to_obj_identifier(&self) -> String;
}

impl<'a> StrExt for &'a str {
    fn to_obj_identifier(&self) -> String {
        self.to_case(Case::Snake).to_uppercase()
    }
}

#[cfg(test)]
pub fn assert_tokens(generated: genco::lang::js::Tokens, compare: genco::lang::js::Tokens) {
    assert_eq!(generated.to_file_string(), compare.to_file_string())
}
