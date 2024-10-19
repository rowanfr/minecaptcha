#![feature(let_chains)]
#![feature(const_trait_impl)]

use win::Win;
use winit::{
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
};

mod egui;
mod egui_render;
mod wgpu;
mod win;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = Win::default();
    // ControlFlow::Wait pauses the event loop if no events are available to process
    // ControlFlow::Poll continuously runs the event loop
    event_loop.set_control_flow(ControlFlow::Wait);

    // Runs the window references as app within the selected event loop.
    event_loop.run_app(&mut app)
}
