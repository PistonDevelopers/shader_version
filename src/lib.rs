#![crate_name = "shader_version"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![feature(globs)]

//! A helper library for detecting and picking compatible shaders.

pub use opengl::OpenGL;

pub mod opengl;
pub mod glsl;

