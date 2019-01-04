use glium;
use glium::Surface;

use crate::shapes::*;

const SPHERE_VERT: &'static str = include_str!("circle.vert");
const SPHERE_FRAG: &'static str = include_str!("circle.frag");
const NSEP: u32 = 30;

#[derive(Copy, Clone)]
pub struct Circle {
    pub coord: (f32, f32, f32),
}

implement_shape_factory!(CircleFactory, Circle, coord);

impl<'a, 'b> ShapeFactoryInfo<'b> for CircleFactory<'a> {
    fn get_vertex_src() -> &'static str {
        SPHERE_VERT
    }

    fn get_fragment_src() -> &'static str {
        SPHERE_FRAG
    }

    fn get_attribute() -> Attribute {
        let mut v = Vec::new();
        for i in 0..NSEP {
            let theta = (i as f32) / (NSEP as f32) * 2.0 * std::f32::consts::PI;
            v.push(Vertex {
                position: [0.5 * theta.cos(), 0.5 * theta.sin(), 0.0],
                color: [0.0, 1.0, 0.0],
                uv: [0.5 * theta.cos(), 0.5 * theta.sin()],
            });
        }
        v
    }

    fn get_indices() -> Indices {
        let mut v = Vec::new();
        for i in 0..NSEP - 2 {
            v.push(i + 1);
            v.push(i + 2);
            v.push(0);
        }
        v
    }

    fn get_draw_parameter() -> glium::DrawParameters<'b> {
        Default::default()
    }
}
