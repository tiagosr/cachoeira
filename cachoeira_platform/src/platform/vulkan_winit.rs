#[macro_use]
extern crate vulkano;

use std::thread;

extern crate winit;

use winit;
use vulkano;
mod render_context;

use super::winit_window;

struct VulkanWinitRenderContext {
    base: render_context::RenderContext,

}



