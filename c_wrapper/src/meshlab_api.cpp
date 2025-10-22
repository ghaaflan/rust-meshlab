/**
 * MeshLab C API Implementation - Mesh Loading
 */

#include "meshlab_api.h"

// MeshLab/VCGlib includes
#include <common/ml_document/cmesh.h>
#include <vcg/complex/algorithms/update/bounding.h>
#include <vcg/complex/algorithms/update/normal.h>
#include <wrap/io_trimesh/import.h>
#include <wrap/io_trimesh/export.h>

#include <vector>
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

// MeshSet: manages a collection of meshes (like PyMeshLab)
struct MeshSetHandle {
    std::vector<CMeshO*> meshes;
    size_t current_mesh_index;

    MeshSetHandle() : current_mesh_index(0) {}

    ~MeshSetHandle() {
        for (auto* mesh : meshes) {
            delete mesh;
        }
    }

    CMeshO* current() {
        if (meshes.empty() || current_mesh_index >= meshes.size()) {
            return nullptr;
        }
        return meshes[current_mesh_index];
    }

    const CMeshO* current() const {
        if (meshes.empty() || current_mesh_index >= meshes.size()) {
            return nullptr;
        }
        return meshes[current_mesh_index];
    }
};

// Single mesh handle (just wraps CMeshO pointer)
struct MeshHandle {
    const CMeshO* mesh;
    MeshHandle(const CMeshO* m) : mesh(m) {}
};

// ============================================================================
// MeshSet Lifecycle
// ============================================================================

MeshSetHandle* meshset_create(void) {
    try {
        clear_error();
        return new MeshSetHandle();
    } catch (const std::exception& e) {
        set_error(std::string("Failed to create MeshSet: ") + e.what());
        return nullptr;
    }
}

void meshset_destroy(MeshSetHandle* ms) {
    if (ms) {
        delete ms;
    }
}

size_t meshset_mesh_count(const MeshSetHandle* ms) {
    if (!ms) return 0;
    return ms->meshes.size();
}

// ============================================================================
// Mesh Loading
// ============================================================================

MeshLabResult meshset_load_mesh(MeshSetHandle* ms, const char* filename) {
    if (!ms || !filename) return MESHLAB_ERROR_NULL_POINTER;

    try {
        clear_error();

        // Create new mesh
        CMeshO* mesh = new CMeshO();

        // Load the file
        int mask = 0;
        int result = vcg::tri::io::Importer<CMeshO>::Open(*mesh, filename, mask);

        if (result != 0) {
            delete mesh;
            set_error(std::string("Failed to load '") + filename + "': " +
                     vcg::tri::io::Importer<CMeshO>::ErrorMsg(result));
            return MESHLAB_ERROR_LOAD_FAILED;
        }

        // Update bounding box and normals
        vcg::tri::UpdateBounding<CMeshO>::Box(*mesh);
        vcg::tri::UpdateNormal<CMeshO>::PerVertexNormalizedPerFace(*mesh);

        // Add to meshset
        ms->meshes.push_back(mesh);
        ms->current_mesh_index = ms->meshes.size() - 1;

        return MESHLAB_SUCCESS;
    } catch (const std::exception& e) {
        set_error(std::string("Load failed: ") + e.what());
        return MESHLAB_ERROR_LOAD_FAILED;
    }
}

MeshLabResult meshset_save_current_mesh(const MeshSetHandle* ms, const char* filename) {
    if (!ms || !filename) return MESHLAB_ERROR_NULL_POINTER;

    const CMeshO* mesh = ms->current();
    if (!mesh) {
        set_error("No current mesh to save");
        return MESHLAB_ERROR_NO_MESH;
    }

    try {
        clear_error();

        int mask = vcg::tri::io::Mask::IOM_VERTCOORD |
                   vcg::tri::io::Mask::IOM_VERTNORMAL |
                   vcg::tri::io::Mask::IOM_FACEINDEX |
                   vcg::tri::io::Mask::IOM_FACENORMAL;

        // Need non-const for Save
        CMeshO& m = const_cast<CMeshO&>(*mesh);
        int result = vcg::tri::io::Exporter<CMeshO>::Save(m, filename, mask);

        if (result != 0) {
            set_error(std::string("Failed to save '") + filename + "': " +
                     vcg::tri::io::Exporter<CMeshO>::ErrorMsg(result));
            return MESHLAB_ERROR_SAVE_FAILED;
        }

        return MESHLAB_SUCCESS;
    } catch (const std::exception& e) {
        set_error(std::string("Save failed: ") + e.what());
        return MESHLAB_ERROR_SAVE_FAILED;
    }
}

// ============================================================================
// Current Mesh Access
// ============================================================================

const MeshHandle* meshset_current_mesh(const MeshSetHandle* ms) {
    if (!ms) return nullptr;

    const CMeshO* mesh = ms->current();
    if (!mesh) return nullptr;

    // Note: This creates a new MeshHandle each time
    // In production, you'd want to manage this better
    return new MeshHandle(mesh);
}

MeshLabResult meshset_set_current_mesh(MeshSetHandle* ms, size_t index) {
    if (!ms) return MESHLAB_ERROR_NULL_POINTER;

    if (index >= ms->meshes.size()) {
        set_error("Mesh index out of range");
        return MESHLAB_ERROR_INVALID_PARAM;
    }

    ms->current_mesh_index = index;
    return MESHLAB_SUCCESS;
}

// ============================================================================
// Mesh Properties
// ============================================================================

size_t mesh_vertex_count(const MeshHandle* mesh) {
    if (!mesh || !mesh->mesh) return 0;
    return mesh->mesh->VN();
}

size_t mesh_face_count(const MeshHandle* mesh) {
    if (!mesh || !mesh->mesh) return 0;
    return mesh->mesh->FN();
}

MeshLabResult mesh_bounding_box(const MeshHandle* mesh, BoundingBox* bbox) {
    if (!mesh || !mesh->mesh || !bbox) return MESHLAB_ERROR_NULL_POINTER;

    try {
        const auto& box = mesh->mesh->bbox;
        bbox->min.x = box.min[0];
        bbox->min.y = box.min[1];
        bbox->min.z = box.min[2];
        bbox->max.x = box.max[0];
        bbox->max.y = box.max[1];
        bbox->max.z = box.max[2];
        return MESHLAB_SUCCESS;
    } catch (const std::exception& e) {
        set_error(e.what());
        return MESHLAB_ERROR_UNKNOWN;
    }
}

// ============================================================================
// Filters
// ============================================================================

MeshLabResult meshset_filter_vertex_displacement(
    MeshSetHandle* ms,
    float max_displacement,
    bool update_normals,
    int random_seed)
{
    if (!ms) return MESHLAB_ERROR_NULL_POINTER;

    CMeshO* mesh = ms->current();
    if (!mesh) {
        set_error("No current mesh");
        return MESHLAB_ERROR_NO_MESH;
    }

    if (max_displacement < 0) return MESHLAB_ERROR_INVALID_PARAM;

    try {
        clear_error();

        // Set random seed
        if (random_seed == 0) {
            srand(static_cast<unsigned int>(time(nullptr)));
        } else {
            srand(static_cast<unsigned int>(random_seed));
        }

        // Displace vertices
        for (size_t i = 0; i < mesh->vert.size(); i++) {
            float rndax = (float(2.0 * rand()) / float(RAND_MAX) - 1.0f) * max_displacement;
            float rnday = (float(2.0 * rand()) / float(RAND_MAX) - 1.0f) * max_displacement;
            float rndaz = (float(2.0 * rand()) / float(RAND_MAX) - 1.0f) * max_displacement;
            mesh->vert[i].P()[0] += rndax;
            mesh->vert[i].P()[1] += rnday;
            mesh->vert[i].P()[2] += rndaz;
        }

        // Update normals if requested
        if (update_normals) {
            vcg::tri::UpdateNormal<CMeshO>::PerVertexNormalizedPerFace(*mesh);
        }

        // Update bounding box
        vcg::tri::UpdateBounding<CMeshO>::Box(*mesh);

        return MESHLAB_SUCCESS;
    } catch (const std::exception& e) {
        set_error(std::string("Vertex displacement failed: ") + e.what());
        return MESHLAB_ERROR_UNKNOWN;
    }
}

// ============================================================================
// Error Handling
// ============================================================================

const char* meshlab_error_string(MeshLabResult result) {
    switch (result) {
        case MESHLAB_SUCCESS: return "Success";
        case MESHLAB_ERROR_NULL_POINTER: return "Null pointer";
        case MESHLAB_ERROR_INVALID_PARAM: return "Invalid parameter";
        case MESHLAB_ERROR_FILE_NOT_FOUND: return "File not found";
        case MESHLAB_ERROR_LOAD_FAILED: return "Failed to load mesh";
        case MESHLAB_ERROR_SAVE_FAILED: return "Failed to save mesh";
        case MESHLAB_ERROR_NO_MESH: return "No mesh available";
        case MESHLAB_ERROR_UNKNOWN: return "Unknown error";
        default: return "Invalid result code";
    }
}

const char* meshlab_last_error(void) {
    if (g_last_error.empty()) {
        return nullptr;
    }
    return g_last_error.c_str();
}
