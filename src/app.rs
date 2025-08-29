use std::sync::Arc;

use tracing::debug;
use winit::{application::ApplicationHandler, window::Window};

use crate::wgpu_ctx::WgpuCtx;

#[derive(Default)]
pub struct App<'window> {
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window>>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = Window::default_attributes().with_title("Chroma");
            let window = Arc::new(event_loop.create_window(attributes).unwrap());
            self.window = Some(window.clone());
            self.wgpu_ctx = Some(WgpuCtx::new(window.clone()).unwrap());
            window.request_redraw();
        }
        debug!("App resumed");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        debug!("Window event: {:?}", event);
        match event {
            winit::event::WindowEvent::Resized(new_size) => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.wgpu_ctx.as_mut(), self.window.as_ref())
                {
                    wgpu_ctx.resize(new_size.into());
                    window.request_redraw();
                }
            }
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Destroyed => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                if let (Some(wgpu_ctx), Some(_window)) =
                    (self.wgpu_ctx.as_mut(), self.window.as_ref())
                {
                    wgpu_ctx.draw().unwrap();
                }
            }
            _ => (),
        }
    }
}
