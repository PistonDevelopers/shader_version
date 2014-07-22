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

/// For OpenGL version 3.3 and above,
/// the GLSL version is the same as the OpenGL version.
///
/// Source: http://www.opengl.org/wiki/Core_Language_%28GLSL%29
#[allow(missing_doc)]
#[allow(non_camel_case_types)]
pub enum GLSL {
    GLSL_1_10,
    GLSL_1_20,
    GLSL_1_30,
    GLSL_1_40,
    GLSL_1_50,
    GLSL_3_30,
    GLSL_4_00,
    GLSL_4_10,
    GLSL_4_20,
    GLSL_4_30,
    GLSL_4_40
}

impl OpenGL {
    /// Gets the minor version of OpenGL.
    pub fn get_major_minor(&self) -> (int, int) {
        match *self {
            OpenGL_2_0 => (2, 0),
            OpenGL_2_1 => (2, 1),
            OpenGL_3_0 => (3, 0),
            OpenGL_3_1 => (3, 1),
            OpenGL_3_2 => (3, 2),
            OpenGL_3_3 => (3, 3),
            OpenGL_4_0 => (4, 0),
            OpenGL_4_1 => (4, 1),
            OpenGL_4_2 => (4, 2),
            OpenGL_4_3 => (4, 3),
            OpenGL_4_4 => (4, 4)
        }
    }

    /// Gets GLSL version associated with OpenGL.
    #[allow(non_snake_case_functions)]
    pub fn to_GLSL(&self) -> GLSL {
        match *self {
            OpenGL_2_0 => GLSL_1_10,
            OpenGL_2_1 => GLSL_1_20,
            OpenGL_3_0 => GLSL_1_30,
            OpenGL_3_1 => GLSL_1_40,
            OpenGL_3_2 => GLSL_1_50,
            OpenGL_3_3 => GLSL_3_30,
            OpenGL_4_0 => GLSL_4_00,
            OpenGL_4_1 => GLSL_4_10,
            OpenGL_4_2 => GLSL_4_20,
            OpenGL_4_3 => GLSL_4_30,
            OpenGL_4_4 => GLSL_4_40,
        }
    }
}

impl GLSL {
    /// Gets OpenGL version associated with GLSL.
    #[allow(non_snake_case_functions)]
    pub fn to_OpenGL(&self) -> OpenGL {
        match *self {
            GLSL_1_10 => OpenGL_2_0,
            GLSL_1_20 => OpenGL_2_1,
            GLSL_1_30 => OpenGL_3_0,
            GLSL_1_40 => OpenGL_3_1,
            GLSL_1_50 => OpenGL_3_2,
            GLSL_3_30 => OpenGL_3_3,
            GLSL_4_00 => OpenGL_4_0,
            GLSL_4_10 => OpenGL_4_1,
            GLSL_4_20 => OpenGL_4_2,
            GLSL_4_30 => OpenGL_4_3,
            GLSL_4_40 => OpenGL_4_4,
        }
    }
}
