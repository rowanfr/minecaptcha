// Briefly vertex shaders devine the visual region fragment shaders act upon by specifying triangles. Fragment shaders specify the pixel color within the pre-defined vertex shader region.

// Vertex shader
// A vertex is a point in 3D space (can also be 2D). These vertices are then bundled in groups of 2s to form lines and/or 3s to form triangles.
// We use a vertex shader to manipulate the vertices in order to transform the shape to look the way we want it.
// Vertex shaders are highly parallelizable. Each vertex is processed independently
/*
One can use a heigh map texture to modify vertex shaders
:
a) Texture Access Patterns:
Just as with fragment shaders, non-uniform texture access in vertex shaders can lead to performance implications. If different vertices access wildly different parts of a texture, it can result in cache thrashing.
b) Dependent Texture Reads:
Vertex shaders can perform dependent texture reads (where the result of one texture read influences the next), which can introduce some serialization and affect parallelism.
c) Vertex Attribute Interpolation:
While not directly related to the vertex shader execution, the outputs of vertex shaders are typically interpolated across primitives, which can introduce some interdependence in the overall pipeline.
d) Compute Shader Alternative:
For complex vertex processing that involves a lot of texture lookups or interdependent calculations, sometimes developers opt to use compute shaders instead, which offer more flexibility in terms of memory access patterns.
*/
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

// Full Screen basic vertex shader. This is less effeciant than the 4 verticy one but its easier to understand and work with. Optimize it later
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

// Fragment shader
// The vertices are then converted into fragments. Every pixel in the result image gets at least one fragment. Each fragment has a color that will be copied to its corresponding pixel. The fragment shader decides what color the fragment will be.
// Fragment shaders are designed to be highly parallelizable, with each fragment's core computation typically being independent. This allows GPUs to process many fragments simultaneously, though certain techniques and memory access patterns can introduce interdependencies that affect performance and execution
/*
Caveats:
a) While fragments can be processed independently, they don't always correspond one-to-one with pixels in the final image. Multiple fragments might contribute to a single pixel (e.g., in the case of transparency or anti-aliasing).
b) Modern fragment shaders can read from textures and perform dependent texture reads, which can introduce some interdependence between fragments.
c) Some advanced techniques (like order-independent transparency) may require fragments to be processed in a specific order or may need to consider multiple fragments together.
GPU Architecture: GPUs typically process fragments in groups (often called "warps" or "wavefronts"). While each fragment's computation is logically independent, divergent execution within these groups can impact performance.
Memory Access: While the computation for each fragment is independent, they may access shared resources (like textures or uniform buffers), which can lead to memory access patterns affecting overall performance.
*/

struct Uniforms {
    mouse_position: vec2<f32>,
    is_mouse_pressed: f32,
    drag_start: vec2<f32>,
    drag_end: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}