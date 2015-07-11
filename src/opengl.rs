//! Models versions of OpenGL

use glsl::GLSL;

#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum OpenGL {
    V2_0,
    V2_1,
    V3_0,
    V3_1,
    V3_2,
    V3_3,
    V4_0,
    V4_1,
    V4_2,
    V4_3,
    V4_4,
    V4_5
}

impl OpenGL {
    /// Gets the minor version of OpenGL.
    pub fn get_major_minor(&self) -> (isize, isize) {
        match *self {
            OpenGL::V2_0 => (2, 0),
            OpenGL::V2_1 => (2, 1),
            OpenGL::V3_0 => (3, 0),
            OpenGL::V3_1 => (3, 1),
            OpenGL::V3_2 => (3, 2),
            OpenGL::V3_3 => (3, 3),
            OpenGL::V4_0 => (4, 0),
            OpenGL::V4_1 => (4, 1),
            OpenGL::V4_2 => (4, 2),
            OpenGL::V4_3 => (4, 3),
            OpenGL::V4_4 => (4, 4),
            OpenGL::V4_5 => (4, 5)
        }
    }

    /// Gets GLSL version associated with OpenGL.
    #[allow(non_snake_case)]
    pub fn to_glsl(&self) -> GLSL {
        match *self {
            OpenGL::V2_0 => GLSL::V1_10,
            OpenGL::V2_1 => GLSL::V1_20,
            OpenGL::V3_0 => GLSL::V1_30,
            OpenGL::V3_1 => GLSL::V1_40,
            OpenGL::V3_2 => GLSL::V1_50,
            OpenGL::V3_3 => GLSL::V3_30,
            OpenGL::V4_0 => GLSL::V4_00,
            OpenGL::V4_1 => GLSL::V4_10,
            OpenGL::V4_2 => GLSL::V4_20,
            OpenGL::V4_3 => GLSL::V4_30,
            OpenGL::V4_4 => GLSL::V4_40,
            OpenGL::V4_5 => GLSL::V4_50
        }
    }
}
