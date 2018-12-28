extern crate glium;

pub fn start() {
    use glium::{glutin, Surface};
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window_size = glium::glutin::dpi::LogicalSize::new(640.0, 480.0);
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(window_size)
        .with_title("Arrows");
    let ctx = glium::glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let mut window_should_close = false;
    while !window_should_close {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => window_should_close = true,
                _ => (),
            },
            _ => (),
        });
    }
}
