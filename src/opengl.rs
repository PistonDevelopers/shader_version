
//! Models versions of OpenGL

use glsl::GLSL;

#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy)]
pub enum OpenGL {
    _2_0,
    _2_1,
    _3_0,
    _3_1,
    _3_2,
    _3_3,
    _4_0,
    _4_1,
    _4_2,
    _4_3,
    _4_4,
    _4_5
}

impl OpenGL {
    /// Gets the minor version of OpenGL.
    pub fn get_major_minor(&self) -> (int, int) {
        match *self {
            OpenGL::_2_0 => (2, 0),
            OpenGL::_2_1 => (2, 1),
            OpenGL::_3_0 => (3, 0),
            OpenGL::_3_1 => (3, 1),
            OpenGL::_3_2 => (3, 2),
            OpenGL::_3_3 => (3, 3),
            OpenGL::_4_0 => (4, 0),
            OpenGL::_4_1 => (4, 1),
            OpenGL::_4_2 => (4, 2),
            OpenGL::_4_3 => (4, 3),
            OpenGL::_4_4 => (4, 4),
            OpenGL::_4_5 => (4, 5)
        }
    }

    /// Gets GLSL version associated with OpenGL.
    #[allow(non_snake_case)]
    pub fn to_GLSL(&self) -> GLSL {
        match *self {
            OpenGL::_2_0 => GLSL::_1_10,
            OpenGL::_2_1 => GLSL::_1_20,
            OpenGL::_3_0 => GLSL::_1_30,
            OpenGL::_3_1 => GLSL::_1_40,
            OpenGL::_3_2 => GLSL::_1_50,
            OpenGL::_3_3 => GLSL::_3_30,
            OpenGL::_4_0 => GLSL::_4_00,
            OpenGL::_4_1 => GLSL::_4_10,
            OpenGL::_4_2 => GLSL::_4_20,
            OpenGL::_4_3 => GLSL::_4_30,
            OpenGL::_4_4 => GLSL::_4_40,
            OpenGL::_4_5 => GLSL::_4_50
        }
    }
}
