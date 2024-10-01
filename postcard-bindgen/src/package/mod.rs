#[cfg(feature = "js")]
mod npm_package;
#[cfg(feature = "python")]
mod pip_module;

#[cfg(feature = "js")]
pub use npm_package::build_npm_package as build_package;
#[cfg(feature = "python")]
pub use pip_module::build_pip_module as build_package;


use std::{
    error::Error,
    fmt::{Debug, Display},
    str::FromStr,
};

/// Defines a package version with major, minor, patch version numbers.
///
/// # Examples
/// ```
/// # use postcard_bindgen::Version;
/// let version = Version::from_array([2, 10, 2]);
/// assert_eq!(version.to_string(), String::from("2.10.2"))
/// ```
///
/// ```
/// # use std::str::FromStr;
/// # use postcard_bindgen::Version;
/// let version = Version::from_str("2.10.2").unwrap();
/// assert_eq!(version.to_string(), String::from("2.10.2"))
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

/// Holds npm package info.
pub struct PackageInfo {
    pub name: String,
    pub version: Version,
}

impl Version {
    pub fn from_array(parts: [u32; 3]) -> Self {
        Self {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        }
    }
}

/// Error type that indicates that the supplied string is not a version formatted string.
pub struct VersionFromStrError;

impl Debug for VersionFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "supplied string not a version format - <major.minor.patch>"
        )
    }
}

impl Display for VersionFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for VersionFromStrError {}

impl FromStr for Version {
    type Err = VersionFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            Err(VersionFromStrError)
        } else {
            Ok(Self {
                major: u32::from_str(parts[0]).map_err(|_| VersionFromStrError)?,
                minor: u32::from_str(parts[1]).map_err(|_| VersionFromStrError)?,
                patch: u32::from_str(parts[2]).map_err(|_| VersionFromStrError)?,
            })
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for Version {
    type Error = VersionFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}