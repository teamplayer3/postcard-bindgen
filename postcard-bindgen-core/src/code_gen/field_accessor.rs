#[derive(Debug, Clone, Copy)]
pub enum FieldAccessor<'a> {
    Object(&'a str),
    Array,
    None,
}
