use chroma::app::App;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();

    event_loop.run_app(&mut app)?;

    Ok(())
}
