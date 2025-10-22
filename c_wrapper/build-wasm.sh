#!/bin/bash
# Build C++ wrapper to WASM using Emscripten

set -e

# Check if emscripten is available
if ! command -v emcc &> /dev/null; then
    echo "Error: Emscripten not found!"
    echo "Install it with:"
    echo "  git clone https://github.com/emscripten-core/emsdk.git"
    echo "  cd emsdk"
    echo "  ./emsdk install latest"
    echo "  ./emsdk activate latest"
    echo "  source ./emsdk_env.sh"
    exit 1
fi

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Create build directory
mkdir -p build-wasm
cd build-wasm

# Get MeshLab path (parent directory by default)
MESHLAB_ROOT="${MESHLAB_ROOT:-../../meshlab-main}"

if [ ! -d "$MESHLAB_ROOT" ]; then
    echo "Error: MeshLab source not found at $MESHLAB_ROOT"
    echo "Set MESHLAB_ROOT environment variable to the correct path"
    exit 1
fi

echo "Building C++ wrapper to WASM..."
echo "MeshLab root: $MESHLAB_ROOT"

# Configure with emscripten
emcmake cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DMESHLAB_ROOT="$MESHLAB_ROOT" \
    -DCMAKE_CXX_FLAGS="-fPIC -s WASM=1 -s ALLOW_MEMORY_GROWTH=1 -s EXPORT_ALL=1 -s LINKABLE=1"

# Build
emmake make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

echo ""
echo "WASM build complete!"
echo "Output: $(pwd)/libmeshlab_api.a"
echo ""
echo "Next steps:"
echo "1. Run 'wasm-pack build --target web' from the rust-meshlab root"
echo "2. Use the generated pkg/ directory in your web project"
