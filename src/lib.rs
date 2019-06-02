#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A helper library for detecting and picking compatible shaders.

extern crate graphics_api_version;

pub use opengl::{OpenGL, ParseOpenGLError};

pub mod opengl;
pub mod glsl;

use std::collections::BTreeMap;

/// Shader picker.
pub struct Shaders<'a, V, S: 'a + ?Sized>(BTreeMap<V, &'a S>);

impl<'a, V, S: ?Sized> Shaders<'a, V, S> where V: PickShader {
    /// Creates a new shader picker.
    pub fn new() -> Self {
        Shaders(BTreeMap::new())
    }

    /// Sets source for a shader version.
    pub fn set(&mut self, version: V, source: &'a S) -> &mut Self {
        self.0.insert(version, source);
        self
    }

    /// Get the closest shader to a shader version.
    pub fn get(&self, version: V) -> Option<&S> {
        version.pick_shader(self)
    }
}

/// Implemented by shader version enums.
pub trait PickShader: Ord + Sized {
    /// Pick shader.
    fn pick_shader<'a, S: ?Sized>(self, shaders: &Shaders<'a, Self, S>) -> Option<&'a S>;
}
