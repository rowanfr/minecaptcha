struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};


// Full Screen basic vertex shader. This is less efficient than the 4 vertices one but its easier to understand and work with. Optimize it later
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    // Define positions for two triangles (6 vertices) to form a full screen quad
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), // Vertex 0 (Bottom-left)
        vec2<f32>(1.0, -1.0),  // Vertex 1 (Bottom-right)
        vec2<f32>(-1.0, 1.0),  // Vertex 2 (Top-left)
        vec2<f32>(-1.0, 1.0),  // Vertex 3 (Top-left, second triangle)
        vec2<f32>(1.0, -1.0),  // Vertex 4 (Bottom-right, second triangle)
        vec2<f32>(1.0, 1.0)    // Vertex 5 (Top-right)
    );

    // Use the vertex index to select the appropriate position
    let pos = positions[in_vertex_index];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0); // Clip-space position

    return out;
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    // Normalize the fragment coordinates to [0, 1] range
    let normalized_coord = frag_coord.xy / vec2<f32>(800.0, 600.0);  // Adjust these values to your viewport size
    
    // Use the normalized coordinates for the red and green channels
    // Add some variation to make it more interesting
    return vec4<f32>(
        normalized_coord.xy,
        0.,
        1.0
    );
}

const mat3: mat3x2<f32> = mat3x2<f32>(
    vec2<f32>(1.0, 0.0),  // first column
    vec2<f32>(0.0, 1.0),  // second column
    vec2<f32>(0.0, 0.0)   // third column
);
