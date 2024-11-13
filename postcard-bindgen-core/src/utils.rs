#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerPath<'a> {
    path: &'a str,
}

impl<'a> ContainerPath<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    /// Will return an iterator over the parts of the path.
    pub fn parts(&self) -> impl Iterator<Item = &'a str> {
        self.path.split("::")
    }
}

impl<'a> From<&'a str> for ContainerPath<'a> {
    fn from(path: &'a str) -> Self {
        Self::new(path)
    }
}

impl<'a> AsRef<ContainerPath<'a>> for ContainerPath<'a> {
    fn as_ref(&self) -> &ContainerPath<'a> {
        self
    }
}
