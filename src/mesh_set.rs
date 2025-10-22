//! MeshSet - manages a collection of meshes

use crate::error::{check_result, MeshLabError, Result};
use crate::ffi;
use crate::mesh::Mesh;
use std::ffi::CString;
use std::path::Path;

/// A collection of meshes, similar to PyMeshLab's MeshSet
pub struct MeshSet {
    handle: *mut ffi::MeshSetHandle,
}

impl MeshSet {
    /// Create a new empty MeshSet
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::meshset_create() };
        if handle.is_null() {
            return Err(MeshLabError::NullPointer);
        }
        Ok(MeshSet { handle })
    }

    /// Load a mesh from a file (OBJ, PLY, etc.)
    pub fn load_mesh<P: AsRef<Path>>(&mut self, filename: P) -> Result<()> {
        let path = filename.as_ref().to_str().ok_or_else(|| {
            MeshLabError::InvalidParameter("Invalid path encoding".to_string())
        })?;

        let c_path = CString::new(path).map_err(|_| {
            MeshLabError::InvalidParameter("Path contains null byte".to_string())
        })?;

        unsafe {
            check_result(ffi::meshset_load_mesh(self.handle, c_path.as_ptr()))?;
        }

        Ok(())
    }

    /// Save the current mesh to a file
    pub fn save_current_mesh<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let path = filename.as_ref().to_str().ok_or_else(|| {
            MeshLabError::InvalidParameter("Invalid path encoding".to_string())
        })?;

        let c_path = CString::new(path).map_err(|_| {
            MeshLabError::InvalidParameter("Path contains null byte".to_string())
        })?;

        unsafe {
            check_result(ffi::meshset_save_current_mesh(
                self.handle,
                c_path.as_ptr(),
            ))?;
        }

        Ok(())
    }

    /// Get the current mesh
    pub fn current_mesh(&self) -> Result<Mesh> {
        let mesh_handle = unsafe { ffi::meshset_current_mesh(self.handle) };
        if mesh_handle.is_null() {
            return Err(MeshLabError::NoMesh);
        }
        Ok(Mesh::new(mesh_handle))
    }

    /// Get the number of meshes in the set
    pub fn mesh_count(&self) -> usize {
        unsafe { ffi::meshset_mesh_count(self.handle) }
    }

    /// Set the current mesh by index
    pub fn set_current_mesh(&mut self, index: usize) -> Result<()> {
        unsafe {
            check_result(ffi::meshset_set_current_mesh(self.handle, index))?;
        }
        Ok(())
    }

    /// Apply vertex displacement filter to current mesh
    pub fn apply_vertex_displacement(
        &mut self,
        max_displacement: f32,
        update_normals: bool,
        random_seed: i32,
    ) -> Result<()> {
        unsafe {
            check_result(ffi::meshset_filter_vertex_displacement(
                self.handle,
                max_displacement,
                update_normals,
                random_seed,
            ))?;
        }
        Ok(())
    }
}

impl Drop for MeshSet {
    fn drop(&mut self) {
        unsafe {
            ffi::meshset_destroy(self.handle);
        }
    }
}

// MeshSet is safe to send between threads
unsafe impl Send for MeshSet {}
