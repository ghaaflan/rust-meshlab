/**
 * MeshLab Filter C API
 *
 * Simple C wrapper for MeshLab filters to enable Rust/WASM bindings
 */

#ifndef MESHLAB_FILTER_H
#define MESHLAB_FILTER_H

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque mesh handle
typedef struct MeshHandle MeshHandle;

// Result codes
typedef enum {
    MESH_OK = 0,
    MESH_ERROR_NULL_POINTER = 1,
    MESH_ERROR_INVALID_PARAM = 2,
    MESH_ERROR_ALLOC_FAILED = 3,
    MESH_ERROR_IO_FAILED = 4,
    MESH_ERROR_UNKNOWN = 99
} MeshResult;

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
} BBox;

// ============================================================================
// Mesh Lifecycle
// ============================================================================

/**
 * Create a new empty mesh
 */
MeshHandle* mesh_create(void);

/**
 * Destroy a mesh and free memory
 */
void mesh_destroy(MeshHandle* mesh);

/**
 * Load mesh from file
 */
MeshResult mesh_load(MeshHandle* mesh, const char* filename);

/**
 * Save mesh to file
 */
MeshResult mesh_save(const MeshHandle* mesh, const char* filename);

// ============================================================================
// Mesh Properties
// ============================================================================

/**
 * Get vertex count
 */
size_t mesh_vertex_count(const MeshHandle* mesh);

/**
 * Get face count
 */
size_t mesh_face_count(const MeshHandle* mesh);

/**
 * Get bounding box
 */
MeshResult mesh_get_bbox(const MeshHandle* mesh, BBox* bbox);

// ============================================================================
// Primitive Creation
// ============================================================================

/**
 * Create a box/cube mesh
 */
MeshResult mesh_create_box(MeshHandle* mesh, float size);

/**
 * Create a sphere mesh
 */
MeshResult mesh_create_sphere(MeshHandle* mesh, float radius, int subdivisions);

// ============================================================================
// FILTER: Vertex Displacement
// ============================================================================

/**
 * Apply random vertex displacement
 *
 * @param mesh - Mesh to modify
 * @param max_displacement - Maximum displacement distance
 * @param update_normals - Whether to recompute normals
 * @param random_seed - Random seed (0 = use time)
 */
MeshResult filter_vertex_displacement(
    MeshHandle* mesh,
    float max_displacement,
    bool update_normals,
    int random_seed
);

// ============================================================================
// Error Handling
// ============================================================================

/**
 * Get error message for result code
 */
const char* mesh_result_string(MeshResult result);

/**
 * Get last error message
 */
const char* mesh_last_error(void);

#ifdef __cplusplus
}
#endif

#endif // MESHLAB_FILTER_H
