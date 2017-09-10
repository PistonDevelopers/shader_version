//! Models versions of OpenGL Shader Language (GLSL)

use { OpenGL, PickShader, Shaders };
use std::str::FromStr;
use std::fmt;
use std::error::Error;

/// For OpenGL version 3.3 and above,
/// the GLSL version is the same as the OpenGL version.
///
/// Source: http://www.opengl.org/wiki/Core_Language_%28GLSL%29
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum GLSL {
    V1_10,
    V1_20,
    V1_30,
    V1_40,
    V1_50,
    V3_30,
    V4_00,
    V4_10,
    V4_20,
    V4_30,
    V4_40,
    V4_50
}

impl GLSL {
    /// Gets OpenGL version associated with GLSL.
    #[allow(non_snake_case)]
    pub fn to_opengl(&self) -> OpenGL {
        match *self {
            GLSL::V1_10 => OpenGL::V2_0,
            GLSL::V1_20 => OpenGL::V2_1,
            GLSL::V1_30 => OpenGL::V3_0,
            GLSL::V1_40 => OpenGL::V3_1,
            GLSL::V1_50 => OpenGL::V3_2,
            GLSL::V3_30 => OpenGL::V3_3,
            GLSL::V4_00 => OpenGL::V4_0,
            GLSL::V4_10 => OpenGL::V4_1,
            GLSL::V4_20 => OpenGL::V4_2,
            GLSL::V4_30 => OpenGL::V4_3,
            GLSL::V4_40 => OpenGL::V4_4,
            GLSL::V4_50 => OpenGL::V4_5
        }
    }
}

impl PickShader for GLSL {
    fn pick_shader<'a, S: ?Sized>(self, shaders: &Shaders<'a, Self, S>) -> Option<&'a S> {
        // OpenGL since 3.2 in core profile doesn't support GLSL lower than 1.50.
        // Since there are no compatible shader in this case, it will return `None`.
        let low = if self < GLSL::V1_50 {
            GLSL::V1_10
        } else {
            GLSL::V1_50
        };
        shaders.0.iter()
               .skip_while(|&(v, _)| *v < low)
               .take_while(|&(v, _)| *v <= self)
               .last().map(|(_, &s)| s)
    }
}


impl FromStr for GLSL {
    type Err = ParseGLSLError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.10" => Ok(GLSL::V1_10),
            "1.20" => Ok(GLSL::V1_20),
            "1.30" => Ok(GLSL::V1_30),
            "1.40" => Ok(GLSL::V1_40),
            "1.50" => Ok(GLSL::V1_50),
            "3.30" => Ok(GLSL::V3_30),
            "4.00" => Ok(GLSL::V4_00),
            "4.10" => Ok(GLSL::V4_10),
            "4.20" => Ok(GLSL::V4_20),
            "4.30" => Ok(GLSL::V4_30),
            "4.40" => Ok(GLSL::V4_40),
            "4.50" => Ok(GLSL::V4_50),
            error => Err(ParseGLSLError(error.into())),
        }
    }
}


/// Represents an error while trying to get `GLSL` from `&str`.
#[derive(Debug)]
pub struct ParseGLSLError(String);

impl fmt::Display for ParseGLSLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` is not a valid GLSL version", self.0)
    }
}

impl Error for ParseGLSLError {
    fn description(&self) -> &str {
        "Invalid GLSL version"
    }
}
