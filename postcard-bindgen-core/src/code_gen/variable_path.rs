use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariablePath<L> {
    pub(super) parts: Vec<VariableAccess>,
    pub(super) start_variable: String,
    _l: PhantomData<L>,
}

impl<L> VariablePath<L> {
    pub fn new(start_variable: String) -> Self {
        Self {
            start_variable,
            parts: Default::default(),
            _l: Default::default(),
        }
    }

    pub fn push(&mut self, part: VariableAccess) {
        self.parts.push(part)
    }

    pub fn modify_push(mut self, part: VariableAccess) -> Self {
        self.push(part);
        self
    }

    pub fn pop(&mut self) -> (&mut Self, Option<VariableAccess>) {
        let popped = self.parts.pop();
        (self, popped)
    }

    pub fn modify_pop(mut self) -> (Self, Option<VariableAccess>) {
        let (_, popped) = self.pop();
        (self, popped)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableAccess {
    Indexed(usize),
    Field(String),
}
