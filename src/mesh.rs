//! Safe wrapper for individual mesh

use crate::error::{check_result, Result};
use crate::ffi;

/// Represents a single mesh in a MeshSet
pub struct Mesh<'a> {
    handle: *const ffi::MeshHandle,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Mesh<'a> {
    pub(crate) fn new(handle: *const ffi::MeshHandle) -> Self {
        Mesh {
            handle,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the number of vertices in the mesh
    pub fn vertex_count(&self) -> usize {
        unsafe { ffi::mesh_vertex_count(self.handle) }
    }

    /// Get the number of faces in the mesh
    pub fn face_count(&self) -> usize {
        unsafe { ffi::mesh_face_count(self.handle) }
    }

    /// Get the bounding box of the mesh
    pub fn bounding_box(&self) -> Result<BoundingBox> {
        let mut bbox = ffi::BoundingBox {
            min: ffi::Point3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: ffi::Point3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        unsafe {
            check_result(ffi::mesh_bounding_box(self.handle, &mut bbox))?;
        }

        Ok(BoundingBox {
            min: Point3f {
                x: bbox.min.x,
                y: bbox.min.y,
                z: bbox.min.z,
            },
            max: Point3f {
                x: bbox.max.x,
                y: bbox.max.y,
                z: bbox.max.z,
            },
        })
    }
}

/// A 3D point
#[derive(Debug, Copy, Clone)]
pub struct Point3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A 3D bounding box
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Point3f,
    pub max: Point3f,
}
