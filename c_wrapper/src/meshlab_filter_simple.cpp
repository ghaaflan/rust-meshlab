/**
 * MeshLab Filter C API Implementation - Simplified Version
 * This version excludes I/O to avoid linking against all MeshLab libraries
 */

#include "meshlab_filter.h"

// MeshLab/VCGlib includes
#include <common/ml_document/cmesh.h>
#include <vcg/complex/algorithms/create/platonic.h>
#include <vcg/complex/algorithms/update/bounding.h>
#include <vcg/complex/algorithms/update/normal.h>
#include <vcg/complex/algorithms/update/position.h>

#include <string>
#include <cstring>
#include <ctime>
#include <cstdlib>

// Thread-local error storage
static thread_local std::string g_last_error;

static void set_error(const std::string& msg) {
    g_last_error = msg;
}

static void clear_error() {
    g_last_error.clear();
}

// Mesh handle wraps CMeshO
struct MeshHandle {
    CMeshO mesh;
    MeshHandle() : mesh() {}
};

// ============================================================================
// Mesh Lifecycle
// ============================================================================

MeshHandle* mesh_create(void) {
    try {
        clear_error();
        return new MeshHandle();
    } catch (const std::exception& e) {
        set_error(std::string("Failed to create mesh: ") + e.what());
        return nullptr;
    } catch (...) {
        set_error("Failed to create mesh: unknown error");
        return nullptr;
    }
}

void mesh_destroy(MeshHandle* mesh) {
    if (mesh) {
        delete mesh;
    }
}

// I/O functions stubbed out for simplicity
MeshResult mesh_load(MeshHandle* mesh, const char* filename) {
    if (!mesh || !filename) return MESH_ERROR_NULL_POINTER;
    set_error("I/O not implemented in simple version");
    return MESH_ERROR_UNKNOWN;
}

MeshResult mesh_save(const MeshHandle* mesh, const char* filename) {
    if (!mesh || !filename) return MESH_ERROR_NULL_POINTER;
    set_error("I/O not implemented in simple version");
    return MESH_ERROR_UNKNOWN;
}

// ============================================================================
// Mesh Properties
// ============================================================================

size_t mesh_vertex_count(const MeshHandle* mesh) {
    if (!mesh) return 0;
    return mesh->mesh.VN();
}

size_t mesh_face_count(const MeshHandle* mesh) {
    if (!mesh) return 0;
    return mesh->mesh.FN();
}

MeshResult mesh_get_bbox(const MeshHandle* mesh, BBox* bbox) {
    if (!mesh || !bbox) return MESH_ERROR_NULL_POINTER;

    try {
        clear_error();
        const auto& box = mesh->mesh.bbox;
        bbox->min.x = box.min[0];
        bbox->min.y = box.min[1];
        bbox->min.z = box.min[2];
        bbox->max.x = box.max[0];
        bbox->max.y = box.max[1];
        bbox->max.z = box.max[2];
        return MESH_OK;
    } catch (const std::exception& e) {
        set_error(e.what());
        return MESH_ERROR_UNKNOWN;
    }
}

// ============================================================================
// Primitive Creation
// ============================================================================

MeshResult mesh_create_box(MeshHandle* mesh, float size) {
    if (!mesh) return MESH_ERROR_NULL_POINTER;
    if (size <= 0) return MESH_ERROR_INVALID_PARAM;

    try {
        clear_error();
        mesh->mesh.Clear();
        // Create box with specified size
        CMeshO::BoxType bbox;
        bbox.min = CMeshO::CoordType(-size/2, -size/2, -size/2);
        bbox.max = CMeshO::CoordType(size/2, size/2, size/2);
        vcg::tri::Box<CMeshO>(mesh->mesh, bbox);
        vcg::tri::UpdateBounding<CMeshO>::Box(mesh->mesh);
        vcg::tri::UpdateNormal<CMeshO>::PerVertexNormalizedPerFace(mesh->mesh);
        return MESH_OK;
    } catch (const std::exception& e) {
        set_error(std::string("Box creation failed: ") + e.what());
        return MESH_ERROR_UNKNOWN;
    }
}

MeshResult mesh_create_sphere(MeshHandle* mesh, float radius, int subdivisions) {
    if (!mesh) return MESH_ERROR_NULL_POINTER;
    if (radius <= 0 || subdivisions < 0) return MESH_ERROR_INVALID_PARAM;

    try {
        clear_error();
        mesh->mesh.Clear();
        vcg::tri::Sphere<CMeshO>(mesh->mesh, subdivisions);

        // Scale to desired radius
        vcg::tri::UpdatePosition<CMeshO>::Scale(mesh->mesh, radius);
        vcg::tri::UpdateBounding<CMeshO>::Box(mesh->mesh);
        vcg::tri::UpdateNormal<CMeshO>::PerVertexNormalizedPerFace(mesh->mesh);
        return MESH_OK;
    } catch (const std::exception& e) {
        set_error(std::string("Sphere creation failed: ") + e.what());
        return MESH_ERROR_UNKNOWN;
    }
}

// ============================================================================
// FILTER: Vertex Displacement
// ============================================================================

MeshResult filter_vertex_displacement(
    MeshHandle* mesh,
    float max_displacement,
    bool update_normals,
    int random_seed)
{
    if (!mesh) return MESH_ERROR_NULL_POINTER;
    if (max_displacement < 0) return MESH_ERROR_INVALID_PARAM;

    try {
        clear_error();
        CMeshO& m = mesh->mesh;

        // Set random seed
        if (random_seed == 0) {
            srand(static_cast<unsigned int>(time(nullptr)));
        } else {
            srand(static_cast<unsigned int>(random_seed));
        }

        // Displace each vertex randomly
        // This is the core algorithm from filter_sample.cpp
        for (size_t i = 0; i < m.vert.size(); i++) {
            float rndax = (float(2.0 * rand()) / float(RAND_MAX) - 1.0f) * max_displacement;
            float rnday = (float(2.0 * rand()) / float(RAND_MAX) - 1.0f) * max_displacement;
            float rndaz = (float(2.0 * rand()) / float(RAND_MAX) - 1.0f) * max_displacement;
            m.vert[i].P()[0] += rndax;
            m.vert[i].P()[1] += rnday;
            m.vert[i].P()[2] += rndaz;
        }

        // Update normals if requested
        if (update_normals) {
            vcg::tri::UpdateNormal<CMeshO>::PerVertexNormalizedPerFace(m);
        }

        // Update bounding box
        vcg::tri::UpdateBounding<CMeshO>::Box(m);

        return MESH_OK;
    } catch (const std::exception& e) {
        set_error(std::string("Vertex displacement failed: ") + e.what());
        return MESH_ERROR_UNKNOWN;
    }
}

// ============================================================================
// Error Handling
// ============================================================================

const char* mesh_result_string(MeshResult result) {
    switch (result) {
        case MESH_OK: return "Success";
        case MESH_ERROR_NULL_POINTER: return "Null pointer";
        case MESH_ERROR_INVALID_PARAM: return "Invalid parameter";
        case MESH_ERROR_ALLOC_FAILED: return "Allocation failed";
        case MESH_ERROR_IO_FAILED: return "I/O operation failed";
        case MESH_ERROR_UNKNOWN: return "Unknown error";
        default: return "Invalid result code";
    }
}

const char* mesh_last_error(void) {
    if (g_last_error.empty()) {
        return nullptr;
    }
    return g_last_error.c_str();
}
