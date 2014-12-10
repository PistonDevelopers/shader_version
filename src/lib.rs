#![crate_name = "shader_version"]
#![deny(missing_docs)]
#![feature(globs)]

//! A helper library for detecting and picking compatible shaders.

pub use opengl::OpenGL;

pub mod opengl;
pub mod glsl;

