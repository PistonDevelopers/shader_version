#![crate_name = "shader_version"]
#![deny(missing_doc)]

//! A helper library for detecting and picking compatible shaders.

/// Models knowledge about version range.
pub enum VersionRange<T> {
    /// The version range is working from a version,
    /// up to but not included another version.
    ///
    /// With other words, from first time it works until it breaks.
    VersionFromTo(T, T),
    /// The version range is working from a version,
    /// and possibly for all newer versions unless
    /// the context it is used implies otherwise.
    VersionFrom(T)
}

