use glium;
use glium::Surface;

use crate::shapes::*;

const SPHERE_VERT: &'static str = include_str!("rectangle.vert");
const SPHERE_FRAG: &'static str = include_str!("rectangle.frag");

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub coord: (f32, f32, f32),
    pub width: f32,
    pub height: f32,
    pub angle: f32,
}

implement_shape_factory!(RectangleFactory, Rectangle, coord, width, height, angle);

impl<'a, 'b> ShapeFactoryInfo<'b> for RectangleFactory<'a> {
    fn get_vertex_src() -> &'static str {
        SPHERE_VERT
    }

    fn get_fragment_src() -> &'static str {
        SPHERE_FRAG
    }

    fn get_attribute() -> Attribute {
        vec![
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
        ]
    }
    fn get_indices() -> Indices {
        vec![0, 1, 2, 1, 2, 3]
    }
    fn get_draw_parameter() -> glium::DrawParameters<'b> {
        Default::default()
    }
}
