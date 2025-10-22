use rust_meshlab::MeshSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MeshLab Rust Example - Load OBJ");
    println!("================================\n");

    // Create a new MeshSet
    let mut ms = MeshSet::new()?;
    println!("Created MeshSet");

    // Load a mesh file
    println!("Loading mesh...");
    ms.load_mesh("examples/cube.obj")?;
    println!("Loaded successfully!");

    // Get info about the current mesh
    let mesh = ms.current_mesh()?;
    println!("\nMesh Info:");
    println!("  Vertices: {}", mesh.vertex_count());
    println!("  Faces: {}", mesh.face_count());

    // Get bounding box
    let bbox = mesh.bounding_box()?;
    println!("\nBounding Box:");
    println!("  Min: ({}, {}, {})", bbox.min.x, bbox.min.y, bbox.min.z);
    println!("  Max: ({}, {}, {})", bbox.max.x, bbox.max.y, bbox.max.z);

    // Apply a filter
    println!("\nApplying vertex displacement filter...");
    ms.apply_vertex_displacement(0.1, true, 42)?;
    println!("Filter applied!");

    // Save the result
    println!("Saving to output.obj...");
    ms.save_current_mesh("output.obj")?;
    println!("Saved successfully!");

    Ok(())
}
