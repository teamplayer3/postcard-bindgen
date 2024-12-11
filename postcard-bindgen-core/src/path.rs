use core::fmt::Display;
use std::borrow::Cow;

use genco::{lang::Lang, tokens::FormatInto};

/// A part of a path which can either be [str] or [String].
pub type Part<'a> = Cow<'a, str>;
/// A full path joined which can either be [str] or [String].
pub type FullPath<'a> = Cow<'a, str>;

/// A path buffer that can be used to build paths.
///
/// The difference between a `PathBuf` and a `Path` is that a `PathBuf` is mutable and can be
/// modified, while a `Path` is immutable and is joined into one but knows how to split it into
/// parts.
///
/// # Example
///
/// ```rust
/// use postcard_bindgen_core::path::PathBuf;
///
/// let mut path = PathBuf::new();
///
/// assert!(path.is_empty());
///
/// path.push("foo");
/// path.push("bar");
/// path.push("baz");
///
/// assert!(!path.is_empty());
/// assert_eq!(path.parts().map(|p| p.as_ref()).collect::<Vec<&str>>(), vec!["foo", "bar", "baz"]);
/// ```
#[derive(Debug, Hash, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct PathBuf<'a> {
    parts: Vec<Part<'a>>,
}

impl Default for PathBuf<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PathBuf<'a> {
    /// Create a new empty path buffer.
    pub fn new() -> Self {
        PathBuf { parts: Vec::new() }
    }

    /// Join a part into the path buffer by consuming the [PathBuf].
    ///
    /// This allows to chain calls to join parts into the path buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use postcard_bindgen_core::path::PathBuf;
    ///
    /// let path = PathBuf::new().join("foo").join("bar").join("baz");
    ///
    /// assert_eq!(path.parts().map(|p| p.as_ref()).collect::<Vec<&str>>(), vec!["foo", "bar", "baz"]);
    /// ```
    pub fn join(mut self, joiner: impl Into<Part<'a>>) -> Self {
        self.parts.push(joiner.into());
        self
    }

    /// Push a part to the end of the path buffer.
    pub fn push(&mut self, part: impl Into<Part<'a>>) {
        self.parts.push(part.into());
    }

    /// Push a part to the front of the path buffer.
    pub fn push_front(&mut self, part: impl Into<Part<'a>>) {
        self.parts.insert(0, part.into());
    }

    /// Pop a part from the end of the path buffer.
    pub fn pop_front(&mut self) {
        if !self.parts.is_empty() {
            let _ = self.parts.remove(0);
        }
    }

    /// Gives an iterator over the parts of the path buffer.
    pub fn parts(&self) -> impl Iterator<Item = &Part<'a>> {
        self.parts.iter()
    }

    /// Check if the path buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Convert the path buffer into a [Path] by consuming the path buffer.
    pub fn into_path<'b>(self, joiner: impl Into<Part<'b>>) -> Path<'a, 'b> {
        let joiner = joiner.into();
        Path {
            path: Some(self.parts.join(joiner.as_ref()).into()),
            joiner,
        }
    }

    /// Flatten the path to the root by simply removing all parts.
    pub fn flatten(&mut self) {
        self.parts.clear();
    }

    /// Convert the path buffer into an owned path buffer by consuming the path buffer.
    pub fn into_owned(self) -> PathBuf<'static> {
        PathBuf {
            parts: self
                .parts
                .into_iter()
                .map(|p| p.into_owned().into())
                .collect(),
        }
    }
}

impl<'a> FromIterator<Part<'a>> for PathBuf<'a> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Part<'a>>,
    {
        PathBuf {
            parts: iter.into_iter().collect(),
        }
    }
}

impl<'a> From<&'a str> for PathBuf<'a> {
    fn from(value: &'a str) -> Self {
        PathBuf {
            parts: vec![Cow::Borrowed(value)],
        }
    }
}

/// A [Path] can be used to represent a path.
///
/// A path is a sequence of parts that are joined together by a joiner. The joiner is a string that
/// is used to split the path into parts and into a [PathBuf].
///
/// # Example
///
/// ```rust
/// use postcard_bindgen_core::path::Path;
///
/// let path = Path::new("foo/bar/baz", "/");
///
/// assert_eq!(path.parts().collect::<Vec<&str>>(), vec!["foo", "bar", "baz"]);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<'a, 'b> {
    path: Option<FullPath<'a>>,
    joiner: Cow<'b, str>,
}

impl<'a, 'b> Path<'a, 'b> {
    /// Create a new path from something which can be converted into a [FullPath] and a joiner
    /// which is either [str] or [String].
    pub fn new(path: impl Into<FullPath<'a>>, joiner: impl Into<Cow<'b, str>>) -> Self {
        Path {
            path: Some(path.into()),
            joiner: joiner.into(),
        }
    }

    /// Gives an iterator over the parts of the path.
    pub fn parts(&self) -> impl Iterator<Item = &str> {
        self.path
            .as_ref()
            .map(|p| p.split(self.joiner.as_ref()))
            .into_iter()
            .flatten()
    }

    /// Flatten the path to the root by simply removing all parts.
    pub fn flatten(&mut self) {
        self.path = None;
    }

    /// Convert the path into a [PathBuf] by consuming the path and splitting it into parts.
    pub fn into_buf(self) -> PathBuf<'a> {
        PathBuf {
            parts: self
                .path
                .map(|p| {
                    p.split(self.joiner.as_ref())
                        .map(|p| p.to_owned().into())
                        .collect::<Vec<Part<'a>>>()
                })
                .unwrap_or_default(),
        }
    }

    /// Check if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.path.as_ref().map(|p| p.is_empty()).unwrap_or(true)
    }

    /// Convert the path into an owned path by consuming the path.
    pub fn into_owned(self) -> Path<'static, 'static> {
        Path {
            joiner: self.joiner.into_owned().into(),
            path: self.path.map(|p| p.into_owned().into()),
        }
    }
}

impl<'a, 'b> From<Path<'a, 'b>> for String {
    fn from(value: Path<'a, 'b>) -> Self {
        value.path.map(|c| c.into_owned()).unwrap_or_default()
    }
}

impl Display for Path<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{}", path)
        } else {
            write!(f, "")
        }
    }
}

impl<L: Lang> FormatInto<L> for Path<'_, '_> {
    fn format_into(self, t: &mut genco::tokens::Tokens<L>) {
        if let Some(path) = &self.path {
            path.format_into(t);
        }
    }
}
