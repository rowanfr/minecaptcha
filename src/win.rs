use crate::wgpu::WgpuState;
use egui_wgpu::ScreenDescriptor;
use std::sync::Arc;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Surface,
};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

/// This stores the main window and associated WGPU state
#[derive(Default)]
pub struct Win {
    window: Option<Arc<Window>>,
    WgpuState: Option<WgpuState>,
}

impl Win {
    pub fn init(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("MineCaptcha"))
                .expect("Couldn't create window"),
        ));
        self.WgpuState = Some(WgpuState::new(self.window.as_ref().unwrap().clone()));
    }
}

impl ApplicationHandler for Win {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init(event_loop);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            // This is the event which closes our window
            WindowEvent::CloseRequested => {
                println!("Close button pressed. Exiting...");
                event_loop.exit();
            }
            // This is the primary way to animate and redraw the image on the screen
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_mut() {
                    if let Some(wgpu_state) = self.WgpuState.as_mut() {
                        // If you alter the screen continuously it will likely cause an Outdated error as the screen itself is different from when you requested it. This is why we add a loop to continuously request until the user stops resizing the screen
                        let output = loop {
                            // Gets screen size and checks if either width or height is 0. The code will panic if either is true so don't
                            let size = window.inner_size();
                            if size.width > 0 && size.height > 0 {
                                // Configure screen surface size based on the current surface and adapter
                                let surf_conf = Surface::get_default_config(
                                    &wgpu_state.surface,
                                    &wgpu_state.adapter,
                                    size.width,
                                    size.height,
                                )
                                .expect("Unable to create resized configuration");
                                wgpu_state.surface.configure(&wgpu_state.device, &surf_conf);

                                // Configure what the screen renders
                                // This grabs a frame from the surface to render to
                                match wgpu_state.surface.get_current_texture() {
                                    Ok(surf_text) => break surf_text,
                                    Err(e) => match e {
                                        wgpu::SurfaceError::Outdated => {
                                            println!("Outdated Surface");
                                            // Optionally, you can add a small delay here to avoid tight loops
                                            // std::thread::sleep(std::time::Duration::from_millis(10));
                                            continue;
                                        }
                                        _ => panic!("Failed to get frame"),
                                    },
                                }
                            }
                        };

                        // This line creates a TextureView with default settings. We need to do this because we want to control how the render code interacts with the texture. This TextureView describes a texture and associated metadata
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        // We also need to create a CommandEncoder to create the actual commands to send to the GPU. Most modern graphics frameworks expect commands to be stored in a command buffer before being sent to the GPU. The encoder builds a command buffer that we can then send to the GPU.
                        let mut encoder = wgpu_state.device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            },
                        );
                        // The reason this is in a seperate block is that begin_render_pass() borrows encoder mutably (aka &mut self). We can't call encoder.finish() until we release that mutable borrow. If we don't do this then we get error `Command encoder is locked by a previously created render/compute pass. Before recording any new commands, the pass must be ended`. you can also use drop(render_pass) to achieve the same effect
                        {
                            // Encodes a single rendered pass of a screen
                            // RenderPassDescriptor describes the attachments of a render pass
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    // Debug Label
                                    label: Some("Render Pass"),
                                    // color_attachments describe where we are going to draw our color to
                                    // RenderPassColorAttachment has view field, which informs wgpu what texture to save the colors to, a resolve_target is the texture that will receive the resolved output. This will be the same as view unless multisampling is enabled, the ops field takes a wgpu::Operations object. This tells wgpu what to do with the colors on the screen (specified by view).
                                    // ! Color is the background color
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                                r: 0.17,
                                                g: 0.60,
                                                b: 0.88,
                                                a: 1.0,
                                            }),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    occlusion_query_set: None,
                                    timestamp_writes: None,
                                });

                            /*
                            Indeces to reduce vertex count aren't working so something is likely misconfigured. Resolve later

                            let indices: [u8; 6] = [0, 1, 2, 1, 3, 2]; // Define the indices for two triangles. Indices are how the triangle orients vertex placement and overlap

                            // Create the index buffer
                            let index_buffer = wgpu_state.device.create_buffer_init(
                                &wgpu::util::BufferInitDescriptor {
                                    label: Some("Index Buffer"),
                                    contents: &indices,
                                    usage: wgpu::BufferUsages::INDEX,
                                },
                            );

                            // Set the index buffer to allow drawing a square with fewer or overlapping verticies
                            render_pass.set_index_buffer(
                                index_buffer.slice(..),
                                wgpu::IndexFormat::Uint32,
                            ); // Set the index buffer
                            */

                            // Set the render pipeline to integrate the shader
                            render_pass.set_pipeline(&wgpu_state.render_pipeline);

                            // ! We tell wgpu to draw something with the given range of vertices and one instance. This is where @builtin(vertex_index) comes from.
                            render_pass.draw(0..6, 0..1);
                        }

                        // Generated size twice. Resolve later
                        let size = window.inner_size();

                        let screen_descriptor = ScreenDescriptor {
                            size_in_pixels: [size.width, size.height],
                            pixels_per_point: wgpu_state.window.scale_factor() as f32,
                        };

                        // Draws egui
                        wgpu_state.draw(&mut encoder, &view, screen_descriptor);

                        // Submits an iterator of the render command buffer to the queue
                        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
                        // Schedule texture to be presented on the owned surface
                        output.present();
                    }
                }
                // This is what actually causes the redraw event to be emitted
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
