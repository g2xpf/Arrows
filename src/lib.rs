#[macro_use]
extern crate glium;
extern crate image;

#[macro_use]
pub mod shapes;

#[macro_use]
pub mod texture;

use crate::shapes::{circle, rectangle, sphere, ShapeFactory};

pub fn start() {
    use glium::{glutin, Surface};
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window_size = glium::glutin::dpi::LogicalSize::new(640.0, 640.0);
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(window_size)
        .with_title("Arrows");
    let ctx = glium::glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let mut sphere_factory = sphere::SphereFactory::new(&display);
    sphere_factory.spawn(sphere::Sphere {
        coord: (0.0, 0.0, 0.0),
    });
    sphere_factory.spawn(sphere::Sphere {
        coord: (-0.5, -0.5, 0.0),
    });

    let texture = texture!(&display, "../images/rectangle.png", image::PNG);

    let mut rectangle_factory = rectangle::RectangleFactory::new(&display);
    rectangle_factory.spawn(rectangle::Rectangle {
        coord: (0.2, 0.2, 0.0),
        width: 0.7,
        height: 0.5,
        angle: std::f32::consts::PI / 4.0,
        tex: &texture,
    });

    let mut circle_factory = circle::CircleFactory::new(&display);
    circle_factory.spawn(circle::Circle {
        coord: (-0.2, 0.5, 0.0),
    });

    let mut window_should_close = false;
    while !window_should_close {
        for rect in rectangle_factory.iter_mut() {
            rect.angle += 0.016666;
            println!("{}", rect.angle);
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        sphere_factory.draw(&mut target);
        circle_factory.draw(&mut target);
        rectangle_factory.draw(&mut target);
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
