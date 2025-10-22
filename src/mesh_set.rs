//! MeshSet - manages a collection of meshes

use crate::error::{check_result, MeshLabError, Result};
use crate::ffi;
use crate::mesh::Mesh;
use std::ffi::CString;
use std::path::Path;

/// Target edge length specification for remeshing operations
///
/// Similar to PyMeshLab's AbsoluteValue and Percentage types
#[derive(Debug, Clone, Copy)]
pub enum TargetLength {
    /// Absolute value in world units
    Absolute(f32),
    /// Percentage of the mesh bounding box diagonal
    Percentage(f32),
}

/// Method for repairing non-manifold edges
///
/// Similar to PyMeshLab's method parameter for meshing_repair_non_manifold_edges()
#[derive(Debug, Clone, Copy)]
pub enum RepairNonManifoldMethod {
    /// Remove faces: iteratively deletes smallest area face until edge becomes 2-manifold
    RemoveFaces = 0,
    /// Split vertices: each non-manifold edge chain becomes a border
    SplitVertices = 1,
}

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

    /// Apply isotropic explicit remeshing filter to current mesh
    ///
    /// Performs explicit remeshing by repeatedly applying edge flip, collapse,
    /// relax and refine operations to regularize size and aspect ratio.
    ///
    /// # Parameters
    /// - `iterations`: Number of iterations (typical: 5-10)
    /// - `adaptive`: Enable adaptive remeshing
    /// - `target_len`: Target edge length for remeshed edges
    /// - `feature_deg`: Minimum angle (degrees) to preserve sharp features (typical: 30)
    /// - `selected_only`: Only remesh selected faces
    /// - `check_surf_dist`: Check surface distance during operations
    /// - `max_surf_dist`: Maximum allowed surface deviation
    /// - `split_flag`: Enable refine step (edge split)
    /// - `collapse_flag`: Enable collapse step
    /// - `swap_flag`: Enable edge swap step
    /// - `smooth_flag`: Enable smoothing step
    /// - `reproject_flag`: Enable reprojection to original surface
    ///
    /// # Example
    /// ```no_run
    /// # use rust_meshlab::MeshSet;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut ms = MeshSet::new()?;
    /// ms.load_mesh("input.obj")?;
    ///
    /// // Simple remeshing with default settings
    /// ms.apply_isotropic_remeshing(
    ///     10,     // iterations
    ///     false,  // adaptive
    ///     0.05,   // target edge length
    ///     30.0,   // feature angle
    ///     false,  // selected only
    ///     false,  // check surf dist
    ///     0.01,   // max surf dist
    ///     true,   // split
    ///     true,   // collapse
    ///     true,   // swap
    ///     true,   // smooth
    ///     true,   // reproject
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply_isotropic_remeshing(
        &mut self,
        iterations: i32,
        adaptive: bool,
        target_len: f32,
        feature_deg: f32,
        selected_only: bool,
        check_surf_dist: bool,
        max_surf_dist: f32,
        split_flag: bool,
        collapse_flag: bool,
        swap_flag: bool,
        smooth_flag: bool,
        reproject_flag: bool,
    ) -> Result<()> {
        unsafe {
            check_result(ffi::meshset_filter_isotropic_remeshing(
                self.handle,
                iterations,
                adaptive,
                target_len,
                feature_deg,
                selected_only,
                check_surf_dist,
                max_surf_dist,
                split_flag,
                collapse_flag,
                swap_flag,
                smooth_flag,
                reproject_flag,
            ))?;
        }
        Ok(())
    }

    /// Apply isotropic explicit remeshing with flexible target length specification
    ///
    /// This is a more flexible version that accepts either absolute or percentage-based
    /// target edge length, similar to PyMeshLab's AbsoluteValue and Percentage types.
    ///
    /// # Parameters
    /// - `iterations`: Number of iterations (typical: 5-10)
    /// - `adaptive`: Enable adaptive remeshing
    /// - `target_len`: Target edge length (Absolute or Percentage)
    /// - `feature_deg`: Minimum angle (degrees) to preserve sharp features (typical: 30)
    /// - `selected_only`: Only remesh selected faces
    /// - `check_surf_dist`: Check surface distance during operations
    /// - `max_surf_dist`: Maximum allowed surface deviation
    /// - `split_flag`: Enable refine step (edge split)
    /// - `collapse_flag`: Enable collapse step
    /// - `swap_flag`: Enable edge swap step
    /// - `smooth_flag`: Enable smoothing step
    /// - `reproject_flag`: Enable reprojection to original surface
    ///
    /// # Example
    /// ```no_run
    /// # use rust_meshlab::{MeshSet, TargetLength};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut ms = MeshSet::new()?;
    /// ms.load_mesh("input.obj")?;
    ///
    /// // Using absolute target length (0.01 world units)
    /// ms.apply_isotropic_remeshing_with_target(
    ///     10,                           // iterations
    ///     false,                        // adaptive
    ///     TargetLength::Absolute(0.01), // 0.01 units
    ///     30.0,                         // feature angle
    ///     false, false, 0.01,
    ///     true, true, true, true, true,
    /// )?;
    ///
    /// // Or using percentage (1% of bounding box diagonal)
    /// ms.apply_isotropic_remeshing_with_target(
    ///     10,                            // iterations
    ///     false,                         // adaptive
    ///     TargetLength::Percentage(1.0), // 1% of diagonal
    ///     30.0,                          // feature angle
    ///     false, false, 0.01,
    ///     true, true, true, true, true,
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply_isotropic_remeshing_with_target(
        &mut self,
        iterations: i32,
        adaptive: bool,
        target_len: TargetLength,
        feature_deg: f32,
        selected_only: bool,
        check_surf_dist: bool,
        max_surf_dist: f32,
        split_flag: bool,
        collapse_flag: bool,
        swap_flag: bool,
        smooth_flag: bool,
        reproject_flag: bool,
    ) -> Result<()> {
        // Calculate actual target length based on mode
        let actual_target_len = match target_len {
            TargetLength::Absolute(value) => value,
            TargetLength::Percentage(percentage) => {
                // Get mesh bounding box to calculate diagonal
                let mesh = self.current_mesh()?;
                let bbox = mesh.bounding_box()?;
                let diag = ((bbox.max.x - bbox.min.x).powi(2)
                    + (bbox.max.y - bbox.min.y).powi(2)
                    + (bbox.max.z - bbox.min.z).powi(2))
                .sqrt();
                diag * (percentage / 100.0)
            }
        };

        // Call the underlying FFI function with calculated absolute value
        unsafe {
            check_result(ffi::meshset_filter_isotropic_remeshing(
                self.handle,
                iterations,
                adaptive,
                actual_target_len,
                feature_deg,
                selected_only,
                check_surf_dist,
                max_surf_dist,
                split_flag,
                collapse_flag,
                swap_flag,
                smooth_flag,
                reproject_flag,
            ))?;
        }
        Ok(())
    }

    /// Apply the "Merge Close Vertices" filter
    ///
    /// Merges vertices that are closer than the specified threshold distance.
    /// This is useful for cleaning up meshes with nearly-duplicate vertices.
    ///
    /// # Arguments
    /// * `threshold` - Maximum distance between vertices to be merged (absolute distance)
    ///
    /// # Example
    /// ```no_run
    /// use rust_meshlab::MeshSet;
    ///
    /// let mut ms = MeshSet::new()?;
    /// ms.load_mesh("input.obj")?;
    ///
    /// // Merge vertices within 0.001 units of each other
    /// ms.apply_merge_close_vertices(0.001)?;
    ///
    /// ms.save_current_mesh("output.obj")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn apply_merge_close_vertices(&mut self, threshold: f32) -> Result<()> {
        unsafe {
            check_result(ffi::meshset_filter_merge_close_vertices(
                self.handle,
                threshold,
            ))?;
        }
        Ok(())
    }

    /// Remove Duplicate Vertices
    ///
    /// Removes vertices that are exactly duplicated (same position).
    /// Unlike `apply_merge_close_vertices`, this only removes exact duplicates,
    /// not vertices within a threshold distance.
    ///
    /// This is useful for cleaning up meshes that have been combined or processed
    /// and may have vertices at identical positions.
    ///
    /// # Example
    /// ```no_run
    /// use rust_meshlab::MeshSet;
    ///
    /// let mut ms = MeshSet::new()?;
    /// ms.load_mesh("input.obj")?;
    ///
    /// // Remove exact duplicate vertices
    /// ms.apply_remove_duplicate_vertices()?;
    ///
    /// ms.save_current_mesh("output.obj")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # PyMeshLab Equivalent
    /// ```python
    /// ms.meshing_remove_duplicate_vertices()
    /// ```
    pub fn apply_remove_duplicate_vertices(&mut self) -> Result<()> {
        unsafe {
            check_result(ffi::meshset_filter_remove_duplicate_vertices(self.handle))?;
        }
        Ok(())
    }

    /// Repair non Manifold Edges
    ///
    /// Removes non-manifold edges (edges shared by more than 2 faces) using one of two methods.
    ///
    /// Non-manifold edges are topological issues where an edge is shared by more than two faces,
    /// which violates the manifold property required by many mesh processing algorithms.
    ///
    /// # Methods
    ///
    /// - `RemoveFaces`: For each non-manifold edge, iteratively deletes the smallest area face
    ///   until the edge becomes 2-manifold
    /// - `SplitVertices`: Each non-manifold edge chain will become a border by splitting vertices
    ///
    /// # Arguments
    /// * `method` - The repair method to use
    ///
    /// # Example
    /// ```no_run
    /// use rust_meshlab::{MeshSet, RepairNonManifoldMethod};
    ///
    /// let mut ms = MeshSet::new()?;
    /// ms.load_mesh("input.obj")?;
    ///
    /// // Repair by removing faces
    /// ms.apply_repair_non_manifold_edges(RepairNonManifoldMethod::RemoveFaces)?;
    ///
    /// // Or repair by splitting vertices
    /// ms.apply_repair_non_manifold_edges(RepairNonManifoldMethod::SplitVertices)?;
    ///
    /// ms.save_current_mesh("output.obj")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # PyMeshLab Equivalent
    /// ```python
    /// # Remove faces method
    /// ms.meshing_repair_non_manifold_edges(method='Remove Faces')
    ///
    /// # Split vertices method
    /// ms.meshing_repair_non_manifold_edges(method='Split Vertices')
    /// ```
    pub fn apply_repair_non_manifold_edges(&mut self, method: RepairNonManifoldMethod) -> Result<()> {
        unsafe {
            check_result(ffi::meshset_filter_repair_non_manifold_edges(
                self.handle,
                method as i32,
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
