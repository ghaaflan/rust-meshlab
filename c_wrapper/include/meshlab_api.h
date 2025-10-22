/**
 * MeshLab C API - Mesh Loading and Management
 *
 * Simplified API modeled after PyMeshLab for mesh I/O and basic operations
 */

#ifndef MESHLAB_API_H
#define MESHLAB_API_H

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle to MeshSet (manages multiple meshes)
typedef struct MeshSetHandle MeshSetHandle;

// Opaque handle to a single Mesh
typedef struct MeshHandle MeshHandle;

// Result codes
typedef enum {
    MESHLAB_SUCCESS = 0,
    MESHLAB_ERROR_NULL_POINTER = 1,
    MESHLAB_ERROR_INVALID_PARAM = 2,
    MESHLAB_ERROR_FILE_NOT_FOUND = 3,
    MESHLAB_ERROR_LOAD_FAILED = 4,
    MESHLAB_ERROR_SAVE_FAILED = 5,
    MESHLAB_ERROR_NO_MESH = 6,
    MESHLAB_ERROR_UNKNOWN = 99
} MeshLabResult;

// 3D Point
typedef struct {
    float x;
    float y;
    float z;
} Point3f;

// Bounding Box
typedef struct {
    Point3f min;
    Point3f max;
} BoundingBox;

// ============================================================================
// MeshSet Lifecycle (like PyMeshLab's MeshSet)
// ============================================================================

/**
 * Create a new MeshSet
 * Similar to: ms = pymeshlab.MeshSet()
 */
MeshSetHandle* meshset_create(void);

/**
 * Destroy MeshSet and free all meshes
 */
void meshset_destroy(MeshSetHandle* ms);

/**
 * Get number of meshes in the set
 */
size_t meshset_mesh_count(const MeshSetHandle* ms);

// ============================================================================
// Mesh Loading (like PyMeshLab's load_new_mesh)
// ============================================================================

/**
 * Load a mesh from file and add it to the MeshSet
 * Similar to: ms.load_new_mesh('model.obj')
 *
 * @param ms - MeshSet handle
 * @param filename - Path to mesh file (.obj, .ply, .stl, etc.)
 * @return MESHLAB_SUCCESS on success
 */
MeshLabResult meshset_load_mesh(MeshSetHandle* ms, const char* filename);

/**
 * Save the current mesh to file
 * Similar to: ms.save_current_mesh('output.obj')
 */
MeshLabResult meshset_save_current_mesh(const MeshSetHandle* ms, const char* filename);

// ============================================================================
// Current Mesh Access
// ============================================================================

/**
 * Get the current mesh from the MeshSet
 * Returns NULL if no mesh is loaded
 */
const MeshHandle* meshset_current_mesh(const MeshSetHandle* ms);

/**
 * Set which mesh is current (by index)
 */
MeshLabResult meshset_set_current_mesh(MeshSetHandle* ms, size_t index);

// ============================================================================
// Mesh Properties
// ============================================================================

/**
 * Get vertex count of a mesh
 */
size_t mesh_vertex_count(const MeshHandle* mesh);

/**
 * Get face count of a mesh
 */
size_t mesh_face_count(const MeshHandle* mesh);

/**
 * Get bounding box of a mesh
 */
MeshLabResult mesh_bounding_box(const MeshHandle* mesh, BoundingBox* bbox);

// ============================================================================
// Filters (Mesh Processing)
// ============================================================================

/**
 * Apply vertex displacement filter to current mesh
 * Similar to: ms.apply_filter('noisy_isosurface', ...)
 */
MeshLabResult meshset_filter_vertex_displacement(
    MeshSetHandle* ms,
    float max_displacement,
    bool update_normals,
    int random_seed
);

// ============================================================================
// Error Handling
// ============================================================================

/**
 * Get human-readable error message
 */
const char* meshlab_error_string(MeshLabResult result);

/**
 * Get last error message (detailed)
 */
const char* meshlab_last_error(void);

#ifdef __cplusplus
}
#endif

#endif // MESHLAB_API_H
