//! Rust bindings for MeshLab mesh processing library
//!
//! This library provides a safe Rust interface to MeshLab's mesh processing
//! capabilities through FFI to the C++ VCGlib. The API is modeled after PyMeshLab.
//!
//! On native platforms, this links to a native shared library.
//! On WASM, this links to the Emscripten-compiled WASM version of the same C++ code.
//!
//! # Example
//!
//! ```no_run
//! use rust_meshlab::MeshSet;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut ms = MeshSet::new()?;
//!     ms.load_mesh("input.obj")?;
//!
//!     let mesh = ms.current_mesh()?;
//!     println!("Vertices: {}", mesh.vertex_count());
//!     println!("Faces: {}", mesh.face_count());
//!
//!     ms.save_current_mesh("output.obj")?;
//!     Ok(())
//! }
//! ```

mod error;
mod ffi;
mod mesh;
mod mesh_set;

pub use error::{MeshLabError, Result};
pub use mesh::{BoundingBox, Mesh, Point3f};
pub use mesh_set::{MeshSet, RepairNonManifoldMethod, TargetLength};

// WASM-specific exports for JavaScript
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod wasm_bindings;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub use wasm_bindings::*;
