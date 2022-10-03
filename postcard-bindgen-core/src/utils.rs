use convert_case::{Case, Casing};

pub trait StringExt {
    fn trim_all(self) -> Self;
    fn to_obj_identifier(&self) -> Self;
}

pub trait StrExt {
    fn to_obj_identifier(&self) -> String;
}

impl StringExt for String {
    fn trim_all(mut self) -> Self {
        self.retain(|c| !c.is_whitespace());
        self
    }

    fn to_obj_identifier(&self) -> Self {
        self.to_case(Case::Snake).to_uppercase()
    }
}

impl<'a> StrExt for &'a str {
    fn to_obj_identifier(&self) -> String {
        self.to_case(Case::Snake).to_uppercase()
    }
}
