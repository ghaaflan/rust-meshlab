//! Low-level FFI bindings to the C API

use std::os::raw::{c_char, c_float, c_int};

// Opaque handles
#[repr(C)]
pub struct MeshSetHandle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MeshHandle {
    _private: [u8; 0],
}

// Result codes
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MeshLabResult {
    Success = 0,
    ErrorNullPointer = 1,
    ErrorInvalidParam = 2,
    ErrorFileNotFound = 3,
    ErrorLoadFailed = 4,
    ErrorSaveFailed = 5,
    ErrorNoMesh = 6,
    ErrorUnknown = 99,
}

// 3D Point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point3f {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

// Bounding Box
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Point3f,
    pub max: Point3f,
}

extern "C" {
    // MeshSet lifecycle
    pub fn meshset_create() -> *mut MeshSetHandle;
    pub fn meshset_destroy(ms: *mut MeshSetHandle);
    pub fn meshset_mesh_count(ms: *const MeshSetHandle) -> usize;

    // Mesh loading
    pub fn meshset_load_mesh(ms: *mut MeshSetHandle, filename: *const c_char) -> MeshLabResult;
    pub fn meshset_save_current_mesh(ms: *const MeshSetHandle, filename: *const c_char) -> MeshLabResult;

    // Current mesh access
    pub fn meshset_current_mesh(ms: *const MeshSetHandle) -> *const MeshHandle;
    pub fn meshset_set_current_mesh(ms: *mut MeshSetHandle, index: usize) -> MeshLabResult;

    // Mesh properties
    pub fn mesh_vertex_count(mesh: *const MeshHandle) -> usize;
    pub fn mesh_face_count(mesh: *const MeshHandle) -> usize;
    pub fn mesh_bounding_box(mesh: *const MeshHandle, bbox: *mut BoundingBox) -> MeshLabResult;

    // Filters
    pub fn meshset_filter_vertex_displacement(
        ms: *mut MeshSetHandle,
        max_displacement: c_float,
        update_normals: bool,
        random_seed: c_int,
    ) -> MeshLabResult;

    // Error handling
    pub fn meshlab_error_string(result: MeshLabResult) -> *const c_char;
    pub fn meshlab_last_error() -> *const c_char;
}
