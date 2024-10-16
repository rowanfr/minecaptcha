use futures::executor::block_on;
use std::sync::Arc;
use wgpu::{Adapter, Device, DeviceDescriptor, Instance, Queue, RenderPipeline, Surface};
use winit::window::Window;

/// This stores the WGPU state for the window
pub struct WgpuState {
    instance: Instance,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    window: Arc<Window>,
    pub render_pipeline: RenderPipeline,
}

impl WgpuState {
    pub fn new(window: Arc<Window>) -> WgpuState {
        // Instance of WGPU
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            flags: wgpu::InstanceFlags::empty(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        // Surface upon which WGPU acts
        let surface: Surface = instance
            .create_surface(window.clone())
            .expect("Unable to get surface from window handle");

        // Handle to physical graphics and/or compute device
        // Block on is me just handling a future lazily, I can likely do something better
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Unable to get adapter");

        // Requests a connection to a physical device, creating a logical device. Returns the Device together with a Queue that executes command buffers.
        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ))
        .expect("Unable to get device and queue");

        // WGSL Shader initialization
        // Alternatively let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/voxel_shader.wgsl").into()),
        });

        // This is the render pipeline layout
        // Vertex shaders are necessary while fragment shaders are not because the rasterization pipeline still expects something to define where the fragment shader runs. In essence vertex shaders at a minimum describe the screen where fragment shaders are run in graphics pipelines
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        //
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                // Entry point is the function we want to point to in the shader
                entry_point: "vs_main",
                // buffers field tells wgpu what type of vertices we want to pass to the vertex shader. We're specifying the vertices in the vertex shader itself, so we'll leave this empty.
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // fragment is optional, so you have to wrap it in Some(). We need it if we want to store color data to the surface
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // The targets field tells wgpu what color outputs it should set up. Currently, we only need one for the surface. We use the surface's format so that copying to it is easy, and we specify that the blending should just replace old pixel data with new data. We also tell wgpu to write to all colors: red, blue, green, and alpha
                    format: surface
                        .get_capabilities(&adapter)
                        .formats
                        .get(0)
                        .expect("No Format Present")
                        .clone(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            //
            primitive: wgpu::PrimitiveState {
                // Using PrimitiveTopology::TriangleList means that every three vertices will correspond to one triangle
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // The front_face and cull_mode fields tell wgpu how to determine whether a given triangle is facing forward or not. FrontFace::Ccw means that a triangle is facing forward if the vertices are arranged in a counter-clockwise direction.
                front_face: wgpu::FrontFace::Ccw,
                // Triangles that are not considered facing forward are culled (not included in the render) as specified by CullMode::Back
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // We're not using a depth/stencil buffer currently, so we leave depth_stencil as None. This will change later.
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // count determines how many samples the pipeline will use. Multisampling is a complex topic, so we won't get into it here.
                mask: !0, // mask specifies which samples should be active. In this case, we are using all of them
                alpha_to_coverage_enabled: false, // alpha_to_coverage_enabled has to do with anti-aliasing. We're not covering anti-aliasing here
            },
            multiview: None, // multiview indicates how many array layers the render attachments can have. We won't be rendering to array textures, so we can set this to None
            cache: None, // cache allows wgpu to cache shader compilation data. Only really useful for Android build targets
        });

        WgpuState {
            instance,
            surface,
            adapter,
            device,
            queue,
            window,
            render_pipeline,
        }
    }
}
