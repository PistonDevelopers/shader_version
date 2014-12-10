//! Models versions of OpenGL Shader Language (GLSL)

use opengl::OpenGL;

/// For OpenGL version 3.3 and above,
/// the GLSL version is the same as the OpenGL version.
///
/// Source: http://www.opengl.org/wiki/Core_Language_%28GLSL%29
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum GLSL {
    _1_10,
    _1_20,
    _1_30,
    _1_40,
    _1_50,
    _3_30,
    _4_00,
    _4_10,
    _4_20,
    _4_30,
    _4_40,
    _4_50
}

impl GLSL {
    /// Gets OpenGL version associated with GLSL.
    #[allow(non_snake_case)]
    pub fn to_OpenGL(&self) -> OpenGL {
        match *self {
            GLSL::_1_10 => OpenGL::_2_0,
            GLSL::_1_20 => OpenGL::_2_1,
            GLSL::_1_30 => OpenGL::_3_0,
            GLSL::_1_40 => OpenGL::_3_1,
            GLSL::_1_50 => OpenGL::_3_2,
            GLSL::_3_30 => OpenGL::_3_3,
            GLSL::_4_00 => OpenGL::_4_0,
            GLSL::_4_10 => OpenGL::_4_1,
            GLSL::_4_20 => OpenGL::_4_2,
            GLSL::_4_30 => OpenGL::_4_3,
            GLSL::_4_40 => OpenGL::_4_4,
            GLSL::_4_50 => OpenGL::_4_5
        }
    }
}

