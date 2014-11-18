
//! Models versions of OpenGL

use glsl::GLSL;

#[allow(non_camel_case_types)]
#[allow(missing_docs)]
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

impl OpenGL {
    /// Gets the minor version of OpenGL.
    pub fn get_major_minor(&self) -> (int, int) {
        match *self {
            OpenGL::OpenGL_2_0 => (2, 0),
            OpenGL::OpenGL_2_1 => (2, 1),
            OpenGL::OpenGL_3_0 => (3, 0),
            OpenGL::OpenGL_3_1 => (3, 1),
            OpenGL::OpenGL_3_2 => (3, 2),
            OpenGL::OpenGL_3_3 => (3, 3),
            OpenGL::OpenGL_4_0 => (4, 0),
            OpenGL::OpenGL_4_1 => (4, 1),
            OpenGL::OpenGL_4_2 => (4, 2),
            OpenGL::OpenGL_4_3 => (4, 3),
            OpenGL::OpenGL_4_4 => (4, 4)
        }
    }

    /// Gets GLSL version associated with OpenGL.
    #[allow(non_snake_case)]
    pub fn to_GLSL(&self) -> GLSL {
        match *self {
            OpenGL::OpenGL_2_0 => GLSL::GLSL_1_10,
            OpenGL::OpenGL_2_1 => GLSL::GLSL_1_20,
            OpenGL::OpenGL_3_0 => GLSL::GLSL_1_30,
            OpenGL::OpenGL_3_1 => GLSL::GLSL_1_40,
            OpenGL::OpenGL_3_2 => GLSL::GLSL_1_50,
            OpenGL::OpenGL_3_3 => GLSL::GLSL_3_30,
            OpenGL::OpenGL_4_0 => GLSL::GLSL_4_00,
            OpenGL::OpenGL_4_1 => GLSL::GLSL_4_10,
            OpenGL::OpenGL_4_2 => GLSL::GLSL_4_20,
            OpenGL::OpenGL_4_3 => GLSL::GLSL_4_30,
            OpenGL::OpenGL_4_4 => GLSL::GLSL_4_40,
        }
    }
}
