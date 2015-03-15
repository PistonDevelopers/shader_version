#![crate_name = "shader_version"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A helper library for detecting and picking compatible shaders.

pub use opengl::OpenGL;

pub mod opengl;
pub mod glsl;

use glsl::GLSL;
use std::collections::BTreeMap;

/// Shader picker.
pub struct Shaders<'a>(BTreeMap<GLSL, &'a str>);

impl<'a> Shaders<'a> {
    /// Creates a new shader picker.
    pub fn new() -> Self {
        Shaders(BTreeMap::new())
    }

    /// Sets source for a shader version.
    pub fn set(&mut self, version: GLSL, source: &'a str) -> &mut Self {
        self.0.insert(version, source);
        self
    }

    /// Get the closest shader to a shader version.
    pub fn get(&self, version: GLSL) -> Option<&str> {
        let low = if version < GLSL::_1_50 {
            GLSL::_1_10
        } else {
            GLSL::_1_50
        };
        self.0.iter()
            .skip_while(|&(v, _)| *v < low)
            .take_while(|&(v, _)| *v <= version)
            .last().map(|(_, &s)| s)
    }
}

