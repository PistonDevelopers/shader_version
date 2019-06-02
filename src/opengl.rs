//! Models versions of OpenGL

use glsl::GLSL;
use std::str::FromStr;
use std::fmt;
use std::error::Error;

use graphics_api_version::Version;

#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
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

impl Into<Version> for OpenGL {
    fn into(self) -> Version {
        let (major, minor) = self.get_major_minor();
        Version::opengl(major as u32, minor as u32)
    }
}

impl OpenGL {
    /// Creates a new `OpenGL` version from graphics API version.
    pub fn from_api(val: Version) -> Option<OpenGL> {
        if val.api == "OpenGL" {
            Some(match (val.major, val.minor) {
                (2, 0) => OpenGL::V2_0,
                (2, 1) => OpenGL::V2_1,
                (3, 0) => OpenGL::V3_0,
                (3, 1) => OpenGL::V3_1,
                (3, 2) => OpenGL::V3_2,
                (3, 3) => OpenGL::V3_3,
                (4, 0) => OpenGL::V4_0,
                (4, 1) => OpenGL::V4_1,
                (4, 2) => OpenGL::V4_2,
                (4, 3) => OpenGL::V4_3,
                (4, 4) => OpenGL::V4_4,
                (4, 5) => OpenGL::V4_5,
                (_, _) => return None,
            })
        } else {
            None
        }
    }

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


impl FromStr for OpenGL {
    type Err = ParseOpenGLError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2.0" => Ok(OpenGL::V2_0),
            "2.1" => Ok(OpenGL::V2_1),
            "3.0" => Ok(OpenGL::V3_0),
            "3.1" => Ok(OpenGL::V3_1),
            "3.2" => Ok(OpenGL::V3_2),
            "3.3" => Ok(OpenGL::V3_3),
            "4.0" => Ok(OpenGL::V4_0),
            "4.1" => Ok(OpenGL::V4_1),
            "4.2" => Ok(OpenGL::V4_2),
            "4.3" => Ok(OpenGL::V4_3),
            "4.4" => Ok(OpenGL::V4_4),
            "4.5" => Ok(OpenGL::V4_5),
            error => Err(ParseOpenGLError{input: error.into()}),
        }
    }
}


/// Represents an error while trying to get `OpenGL` from `&str`.
#[derive(Debug)]
pub struct ParseOpenGLError{
    input: String
}

impl fmt::Display for ParseOpenGLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` is not a valid OpenGL version", self.input)
    }
}

impl Error for ParseOpenGLError {
    fn description(&self) -> &str {
        "Invalid OpenGL version"
    }
}
