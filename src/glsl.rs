//! Models versions of OpenGL Shader Language (GLSL)

use opengl::OpenGL;

/// For OpenGL version 3.3 and above,
/// the GLSL version is the same as the OpenGL version.
///
/// Source: http://www.opengl.org/wiki/Core_Language_%28GLSL%29
#[allow(missing_docs)]
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
    GLSL_4_40,
    GLSL_4_50
}

impl GLSL {
    /// Gets OpenGL version associated with GLSL.
    #[allow(non_snake_case)]
    pub fn to_OpenGL(&self) -> OpenGL {
        match *self {
            GLSL::GLSL_1_10 => OpenGL::OpenGL_2_0,
            GLSL::GLSL_1_20 => OpenGL::OpenGL_2_1,
            GLSL::GLSL_1_30 => OpenGL::OpenGL_3_0,
            GLSL::GLSL_1_40 => OpenGL::OpenGL_3_1,
            GLSL::GLSL_1_50 => OpenGL::OpenGL_3_2,
            GLSL::GLSL_3_30 => OpenGL::OpenGL_3_3,
            GLSL::GLSL_4_00 => OpenGL::OpenGL_4_0,
            GLSL::GLSL_4_10 => OpenGL::OpenGL_4_1,
            GLSL::GLSL_4_20 => OpenGL::OpenGL_4_2,
            GLSL::GLSL_4_30 => OpenGL::OpenGL_4_3,
            GLSL::GLSL_4_40 => OpenGL::OpenGL_4_4,
            GLSL::GLSL_4_50 => OpenGL::OpenGL_4_5
        }
    }
}
