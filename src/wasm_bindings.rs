//! WASM bindings for JavaScript interop
//!
//! These bindings allow the rust-meshlab library to be used from JavaScript in the browser.

use wasm_bindgen::prelude::*;
use crate::{MeshSet as NativeMeshSet, MeshLabError};

/// WASM-exported MeshSet for JavaScript
#[wasm_bindgen]
pub struct WasmMeshSet {
    inner: NativeMeshSet,
}

#[wasm_bindgen]
impl WasmMeshSet {
    /// Create a new empty MeshSet
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmMeshSet, JsValue> {
        NativeMeshSet::new()
            .map(|inner| WasmMeshSet { inner })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Load a mesh from OBJ string data
    #[wasm_bindgen(js_name = loadFromObjString)]
    pub fn load_from_obj_string(&mut self, obj_data: &str) -> Result<(), JsValue> {
        // For WASM, we'll need to save the string to a temporary location
        // This is a simplified version - in production you'd use the Emscripten FS API
        Err(JsValue::from_str("loadFromObjString not yet implemented for WASM - use loadFromArrayBuffer"))
    }

    /// Get the number of meshes in the set
    #[wasm_bindgen(js_name = meshCount)]
    pub fn mesh_count(&self) -> usize {
        self.inner.mesh_count()
    }

    /// Get vertex count of the current mesh
    #[wasm_bindgen(js_name = vertexCount)]
    pub fn vertex_count(&self) -> Result<usize, JsValue> {
        self.inner
            .current_mesh()
            .map(|m| m.vertex_count())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get face count of the current mesh
    #[wasm_bindgen(js_name = faceCount)]
    pub fn face_count(&self) -> Result<usize, JsValue> {
        self.inner
            .current_mesh()
            .map(|m| m.face_count())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Apply isotropic explicit remeshing
    #[wasm_bindgen(js_name = applyIsotropicRemeshing)]
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
    ) -> Result<(), JsValue> {
        self.inner
            .apply_isotropic_remeshing(
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
            )
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Apply vertex displacement (add noise)
    #[wasm_bindgen(js_name = applyVertexDisplacement)]
    pub fn apply_vertex_displacement(
        &mut self,
        max_displacement: f32,
        update_normals: bool,
        seed: i32,
    ) -> Result<(), JsValue> {
        self.inner
            .apply_vertex_displacement(max_displacement, update_normals, seed)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Merge close vertices
    ///
    /// Merges vertices that are closer than the specified threshold distance.
    /// Useful for cleaning up meshes with nearly-duplicate vertices.
    #[wasm_bindgen(js_name = applyMergeCloseVertices)]
    pub fn apply_merge_close_vertices(&mut self, threshold: f32) -> Result<(), JsValue> {
        self.inner
            .apply_merge_close_vertices(threshold)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Export current mesh to OBJ string
    #[wasm_bindgen(js_name = toObjString)]
    pub fn to_obj_string(&self) -> Result<String, JsValue> {
        // This would need Emscripten FS API implementation
        Err(JsValue::from_str("toObjString not yet implemented for WASM"))
    }
}

/// Initialize the WASM module (optional, for future setup)
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages in browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Get version information
#[wasm_bindgen]
pub fn version() -> String {
    format!(
        "rust-meshlab {} (WASM build with Emscripten)",
        env!("CARGO_PKG_VERSION")
    )
}
