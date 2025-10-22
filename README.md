# rust-meshlab

Rust bindings for MeshLab mesh processing library. Provides a safe, idiomatic Rust API modeled after [PyMeshLab](https://pymeshlab.readthedocs.io/).

## Features

- Load and save 3D meshes (OBJ, PLY formats)
- Access mesh properties (vertices, faces, bounding box)
- Apply mesh processing filters
- Safe Rust wrapper around C/C++ MeshLab core

## Quick Start

```rust
use rust_meshlab::MeshSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mesh set
    let mut ms = MeshSet::new()?;

    // Load a mesh
    ms.load_mesh("input.obj")?;

    // Get mesh info
    let mesh = ms.current_mesh()?;
    println!("Vertices: {}", mesh.vertex_count());
    println!("Faces: {}", mesh.face_count());

    // Apply a filter
    ms.apply_vertex_displacement(0.1, true, 42)?;

    // Save result
    ms.save_current_mesh("output.obj")?;

    Ok(())
}
```

## Building

### Prerequisites

- Rust (latest stable)
- CMake 3.18+
- C++11 compiler
- Eigen3 library
- MeshLab source code

### Build Steps

1. Clone MeshLab source to parent directory:
```bash
cd ..
git clone --recursive https://github.com/cnr-isti-vclab/meshlab meshlab-main
cd rust-meshlab
```

2. Install Eigen3:
```bash
# macOS
brew install eigen

# Ubuntu/Debian
sudo apt-get install libeigen3-dev
```

3. Build the C wrapper:
```bash
cd c_wrapper
mkdir build && cd build
cmake ..
make
cd ../..
```

4. Set library path (macOS):
```bash
export DYLD_LIBRARY_PATH=$PWD/c_wrapper/build:$DYLD_LIBRARY_PATH
```

For Linux, use:
```bash
export LD_LIBRARY_PATH=$PWD/c_wrapper/build:$LD_LIBRARY_PATH
```

5. Build and run example:
```bash
cargo run --example load_obj
```

## API Overview

### MeshSet

The main interface for working with meshes, managing a collection of meshes:

```rust
let mut ms = MeshSet::new()?;
ms.load_mesh("model.obj")?;
ms.save_current_mesh("output.obj")?;
let count = ms.mesh_count();
ms.set_current_mesh(0)?;
```

### Mesh

Access properties of an individual mesh:

```rust
let mesh = ms.current_mesh()?;
let v_count = mesh.vertex_count();
let f_count = mesh.face_count();
let bbox = mesh.bounding_box()?;
```

### Filters

Apply mesh processing operations:

```rust
ms.apply_vertex_displacement(
    0.1,    // max displacement
    true,   // update normals
    42      // random seed
)?;
```

## Supported File Formats

- OBJ (Wavefront)
- PLY (Stanford)

More formats can be added by linking additional MeshLab I/O modules.

## Architecture

```
User Code (Rust)
    ↓
rust-meshlab (Safe Rust API)
    ↓
FFI Bindings (src/ffi.rs)
    ↓
C Wrapper (c_wrapper/meshlab_api.cpp)
    ↓
VCGlib / MeshLab Core (C++)
```

## Comparison with PyMeshLab

Similar API design for easy transition:

| PyMeshLab | rust-meshlab |
|-----------|--------------|
| `ms = pymeshlab.MeshSet()` | `let mut ms = MeshSet::new()?;` |
| `ms.load_new_mesh('file.obj')` | `ms.load_mesh("file.obj")?;` |
| `ms.current_mesh()` | `ms.current_mesh()?;` |
| `ms.save_current_mesh('out.obj')` | `ms.save_current_mesh("out.obj")?;` |

## License

GPL (following MeshLab's license)

## Credits

- MeshLab: https://github.com/cnr-isti-vclab/meshlab
- VCGlib: http://vcg.isti.cnr.it/vcglib/
