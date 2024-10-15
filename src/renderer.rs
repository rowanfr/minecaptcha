// src/renderer.rs
use egui_wgpu::renderer::ScreenDescriptor;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceError};

// Renderer struct holds all our rendering state
pub struct Renderer {
    pub surface: Surface,
    pub config: SurfaceConfiguration,
    pub egui_wgpu_renderer: egui_wgpu::Renderer,
}

// Helper function to create a new renderer
impl Renderer {
    pub fn new(device: &Device, surface: Surface, width: u32, height: u32) -> Self {
        let surface_format = surface.get_preferred_format(&device.adapter()).unwrap();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(device, &config);

        // Create the egui-wgpu renderer that handles UI rendering
        let egui_wgpu_renderer = egui_wgpu::Renderer::new(device, surface_format, 1);

        Self {
            surface,
            config,
            egui_wgpu_renderer,
        }
    }

    // Resize the surface if the window is resized
    pub fn resize(&mut self, new_width: u32, new_height: u32, device: &Device) {
        self.config.width = new_width;
        self.config.height = new_height;
        self.surface.configure(device, &self.config);
    }

    // Render the next frame
    pub fn render(
        &mut self,
        egui_ctx: &egui::CtxRef,
        device: &Device,
        queue: &Queue,
    ) -> Result<(), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Encode commands to render
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: 1.0,
        };

        // Render egui UI
        self.egui_wgpu_renderer.update_buffers(
            device,
            queue,
            &mut encoder,
            &egui_ctx.tessellate(),
            &screen_descriptor,
        );
        self.egui_wgpu_renderer
            .render(&mut encoder, &view, &screen_descriptor);

        // Submit commands to the GPU
        queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
