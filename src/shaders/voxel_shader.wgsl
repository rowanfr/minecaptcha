// Define camera parameters
struct Camera {
    position: vec3<f32>,   // Camera position in world space
    direction: vec3<f32>,  // Ray direction
    invResolution: vec2<f32>, // Inverse screen resolution
};

// Define camera parameters
struct Screen {
    width: f32,      // Screen width
    height: f32,     // Screen height
};

struct Voxel {
    color: vec3<f32>,    // Voxel color or material
    isSolid: u32,        // Whether this voxel is solid (1) or empty (0)
};

// Fixed size for the voxel grid (8x8x8 = 512)
const GRID_SIZE: u32 = 8u;
const TOTAL_VOXELS: u32 = GRID_SIZE * GRID_SIZE * GRID_SIZE;

struct VoxelGrid {
    voxels: array<Voxel, TOTAL_VOXELS>,  // Fixed-size array of 512 voxels
    position: vec3<f32>,   // Voxel Grid position in world space
};

struct RayMarchingSystem {
    camera: Camera,
    voxelGrid: VoxelGrid,
    screen: Screen,
};

fn getVoxelIndex(x: u32, y: u32, z: u32) -> u32 {
    return x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE;
}

// Full Screen basic vertex shader. This is less efficient than the 4 vertices one but its easier to understand and work with. Optimize it later

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> @builtin(position) vec4<f32> {

    // Define positions for two triangles (6 vertices) to form a full screen quad
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), // Vertex 0 (Bottom-left)
        vec2<f32>(1.0, -1.0),  // Vertex 1 (Bottom-right)
        vec2<f32>(-1.0, 1.0),  // Vertex 2 (Top-left)
        vec2<f32>(-1.0, 1.0),  // Vertex 3 (Top-left, second triangle)
        vec2<f32>(1.0, -1.0),  // Vertex 4 (Bottom-right, second triangle)
        vec2<f32>(1.0, 1.0)    // Vertex 5 (Top-right)
    );

    // Just outputting the basic pixel positions. I expect everything to be done in the fragment shader with raymarching
    let out = vec4<f32>(positions[in_vertex_index],0.0,1.0);
    return out;
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    // Passed in vector has x and y pixel positions of input
    let normalized_coord = frag_coord.xy / vec2<f32>(100.0, 100.0);  // Adjust these values to your viewport size

    if frag_coord[1] > 100 {
        discard;
    }
    
    // Use the normalized coordinates for the red and green channels
    // Add some variation to make it more interesting
    return vec4<f32>(
        normalized_coord.xy,
        0.,
        1.
    );
}

