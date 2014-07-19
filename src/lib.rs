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

#[allow(non_camel_case_types)]
#[allow(missing_doc)]
pub enum OpenGL {
    OpenGL_2_0,
    OpenGL_2_1,
    OpenGL_3_0,
    OpenGL_3_1,
    OpenGL_3_2,
    OpenGL_3_3,
    OpenGL_4_0,
    OpenGL_4_1,
    OpenGL_4_2,
    OpenGL_4_3,
    OpenGL_4_4
}


