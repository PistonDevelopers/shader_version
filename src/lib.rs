#![crate_name = "shader_version"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A helper library for detecting and picking compatible shaders.

pub use opengl::OpenGL;

pub mod opengl;
pub mod glsl;

use std::collections::BTreeMap;

/// Shader picker.
pub struct Shaders<'a, T: PickShader = glsl::GLSL>(BTreeMap<T, &'a str>);

impl<'a, T: PickShader> Shaders<'a, T> {
    /// Creates a new shader picker.
    pub fn new() -> Self {
        Shaders(BTreeMap::new())
    }

    /// Sets source for a shader version.
    pub fn set(&mut self, version: T, source: &'a str) -> &mut Self {
        self.0.insert(version, source);
        self
    }

    /// Get the closest shader to a shader version.
    pub fn get(&self, version: T) -> Option<&str> {
        version.pick_shader(self.0.iter())
    }
}

/// Implemented by shader enums.
pub trait PickShader: Ord {
    /// Pick shader.
    fn pick_shader<'a, I>(&self, map: I) -> Option<&'a str>
        where
            I: Iterator<Item = (&'a Self, &'a &'a str)>;
}

