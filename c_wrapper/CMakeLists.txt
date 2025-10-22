cmake_minimum_required(VERSION 3.18)
project(meshlab_api C CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define MESHLAB_SCALAR as float (required by MeshLab)
add_definitions(-DMESHLAB_SCALAR=float)

# Find MeshLab sources
set(MESHLAB_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." CACHE PATH "Path to MeshLab root")

# Find Eigen3
find_package(Eigen3 REQUIRED)

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${MESHLAB_ROOT}/src
    ${MESHLAB_ROOT}/src/vcglib
    ${EIGEN3_INCLUDE_DIR}
)

# Source files
set(SOURCES
    src/meshlab_api.cpp
    ${MESHLAB_ROOT}/src/common/ml_document/cmesh.cpp
    ${MESHLAB_ROOT}/src/vcglib/wrap/ply/plylib.cpp
)

set(HEADERS
    include/meshlab_api.h
)

# Create shared library
add_library(meshlab_api SHARED ${SOURCES} ${HEADERS})

# Install targets
install(TARGETS meshlab_api
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(FILES ${HEADERS} DESTINATION include)
