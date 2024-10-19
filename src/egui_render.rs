use std::sync::Arc;

use egui::{Context, Shadow, Visuals};
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::State;
use transform_gizmo_egui::{mint::Quaternion, mint::Vector3, Gizmo, GizmoResult};
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::{event::WindowEvent, window::Window};

/// This is the state for the EGUI application that we can use for informing how our shaders operate
pub struct AppState {
    pub gizmo: Gizmo,
    pub rotation: Quaternion<f64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            gizmo: Gizmo::default(),
            rotation: Quaternion {
                v: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                s: 1.0,
            },
        }
    }
}

/// This stores the EGUI state for the window
pub struct EguiRenderer {
    pub context: Context,
    state: State,
    window: Arc<Window>,
    renderer: Renderer,
    app_state: AppState,
}

impl EguiRenderer {
    pub fn new(device: &Device, window: Arc<Window>) -> Self {
        // Egui initializaiton. This is the first thing you need when working with egui. Context contains the InputState, Memory, PlatformOutput, and more.
        let ctx = Context::default();
        let id = ctx.viewport_id();

        // Controls the visual style of egui
        const BORDER_RADIUS: f32 = 2.0;
        let visuals = Visuals {
            window_rounding: egui::Rounding::same(BORDER_RADIUS),
            window_shadow: Shadow::NONE,
            // menu_rounding: todo!(),
            ..Default::default()
        };

        ctx.set_visuals(visuals);

        // This is the basic state that handles integration between winit and egui
        let egui_state = State::new(ctx.clone(), id, &window, None, None, None);

        // These are the settings for the rendered. The format needed, dithering and sampling applied, etc... This is the simplest render possible
        // ! THE TEXTURE FORMAT SHOULD NOT BE HARDCODED AND SHOULD DYNAMICALLY ADJUST TO WHAT'S IN THE PIPELINE
        let egui_renderer = Renderer::new(device, TextureFormat::Bgra8UnormSrgb, None, 1, false);

        EguiRenderer {
            context: ctx,
            state: egui_state,
            window,
            renderer: egui_renderer,
            app_state: AppState::new(),
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        // Static for let mut rpass = encoder.begin_render_pass(&render_pass_descriptor);
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
        mut run_ui: impl FnMut(&Context, &mut AppState),
    ) {
        // self.state.set_pixels_per_point(window.scale_factor() as f32);
        let raw_input = self.state.take_egui_input(window);
        let full_output = self.context.run(raw_input, |_ui| {
            run_ui(&self.context, &mut self.app_state);
        });

        self.state
            .handle_platform_output(window, full_output.platform_output);

        let tris = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }
        self.renderer
            .update_buffers(device, queue, encoder, &tris, &screen_descriptor);
        {
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: window_surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            };

            let render_pass_descriptor = wgpu::RenderPassDescriptor {
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
                label: Some("egui main render pass"),
                timestamp_writes: None,
                occlusion_query_set: None,
            };

            // No idea what nonsense I used to get it to compile
            let render_pass = encoder.begin_render_pass(&render_pass_descriptor);
            let mut render_pass_static: wgpu::RenderPass<'static> =
                unsafe { std::mem::transmute(render_pass.forget_lifetime()) };

            // Call the render method
            self.renderer
                .render(&mut render_pass_static, &tris, &screen_descriptor)
        }
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }
    }
}
