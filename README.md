# rust-meshlab

Rust bindings for MeshLab mesh processing library. Provides a safe, idiomatic Rust API modeled after [PyMeshLab](https://pymeshlab.readthedocs.io/).

> ✅ **Platform Support**:
> - **Native**: Linux, macOS, Windows (fully supported)
> - **WASM**: Browser support via Emscripten (see [WASM_BUILD.md](WASM_BUILD.md))

## Features

- Load and save 3D meshes (OBJ, PLY formats)
- Access mesh properties (vertices, faces, bounding box)
- Apply mesh processing filters (vertex displacement, isotropic remeshing)
- Safe Rust wrapper around C/C++ MeshLab core using VCGlib
- PyMeshLab-compatible API design
- **WASM support** - Same algorithms run in the browser via Emscripten
- Exact feature parity between native and WASM builds

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

## Platform Support

### Native Platforms (✅ Fully Supported)

Works on Linux, macOS, and Windows with the following requirements:
- CMake 3.18+
- C++11 compiler
- Eigen3 library
- MeshLab source code (for building C wrapper)

**Installation:**
```toml
[dependencies]
rust-meshlab = { git = "https://github.com/yourusername/rust-meshlab" }
```

See [QUICK_START.md](QUICK_START.md) for detailed native build instructions.

### WebAssembly (✅ Supported via Emscripten)

WASM support is implemented using Emscripten to compile the C++ VCGlib to WebAssembly. This provides:
- ✅ **Exact same algorithms** as native builds
- ✅ **Full feature parity** - all filters work identically
- ✅ **Browser compatibility** - runs in any modern browser
- ✅ **No code duplication** - single C++ implementation

**For JavaScript/Browser usage:**
```javascript
import init, { WasmMeshSet } from 'rust-meshlab';

await init();
const ms = new WasmMeshSet();
await ms.applyIsotropicRemeshing(10, false, 0.1, 30.0, ...);
console.log('Vertices:', ms.vertexCount());
```

**Performance:** WASM builds achieve 70-90% of native performance.

See [WASM_BUILD.md](WASM_BUILD.md) for complete build instructions and usage examples.

### Publishing Status

**Not on crates.io yet** because:
1. Requires local C++ library build (not auto-downloadable)
2. Build process requires external dependencies (Eigen3, MeshLab source)

**Current usage:**
- **Rust**: Add as Git dependency (see above)
- **JavaScript**: Build locally and use from `pkg/` directory, or publish to NPM

See [PUBLISHING.md](PUBLISHING.md) for details on future crates.io/NPM publication.

## Available Filters

Currently implemented filters:
- **Vertex Displacement**: Add random noise to mesh vertices
- **Isotropic Explicit Remeshing**: Regularize mesh triangulation with uniform edge lengths
- **Merge Close Vertices**: Merge vertices that are closer than a specified threshold

More filters coming soon! The architecture supports easy addition of any VCGlib/MeshLab filter.

## License

GPL-3.0-or-later (following MeshLab's license)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

## Credits

- MeshLab: https://github.com/cnr-isti-vclab/meshlab
- VCGlib: http://vcg.isti.cnr.it/vcglib/
- PyMeshLab: https://pymeshlab.readthedocs.io/ (API inspiration)
