//! Rust bindings for MeshLab mesh processing library
//!
//! This library provides a safe Rust interface to MeshLab's mesh processing
//! capabilities through a C wrapper. The API is modeled after PyMeshLab.
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
pub use mesh_set::MeshSet;
