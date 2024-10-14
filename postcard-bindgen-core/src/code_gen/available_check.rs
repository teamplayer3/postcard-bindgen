use super::variable_path::{VariableAccess, VariablePath};

#[derive(Clone)]
pub enum AvailableCheck<L> {
    Object(VariablePath<L>, String),
    None,
}

impl<L> AvailableCheck<L> {
    pub fn from_variable_path(path: VariablePath<L>) -> Self {
        let (path, last) = path.modify_pop();
        match last {
            Some(VariableAccess::Field(name)) => Self::Object(path, name),
            _ => Self::None,
        }
    }
}
