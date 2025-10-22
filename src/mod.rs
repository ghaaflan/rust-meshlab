//! Native platform implementation using FFI to C++ MeshLab library

mod error;
mod ffi;
mod mesh;
mod mesh_set;

pub use error::{MeshLabError, Result};
pub use mesh::{BoundingBox, Mesh, Point3f};
pub use mesh_set::MeshSet;
