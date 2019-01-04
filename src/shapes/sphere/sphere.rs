use glium;
use glium::Surface;

use crate::shapes::{Attribute, Index, Indices, ShapeFactory, ShapeFactoryInfo, Vertex};
use cgmath;

const SPHERE_VERT: &'static str = include_str!("sphere.vert");
const SPHERE_FRAG: &'static str = include_str!("sphere.frag");

#[derive(Copy, Clone)]
pub struct Sphere {
    pub coord: (f32, f32, f32),
}

implement_shape_factory!(SphereFactory, Sphere, coord);

impl<'a, 'b> ShapeFactoryInfo<'b> for SphereFactory<'a> {
    fn get_vertex_src() -> &'static str {
        SPHERE_VERT
    }

    fn get_fragment_src() -> &'static str {
        SPHERE_FRAG
    }

    fn get_attribute() -> Attribute {
        vec![
            Vertex {
                position: [0.5, 0.5, 1.0],
                color: [0.0, 0.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.0, 1.0],
                color: [0.0, 1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, 1.0],
                color: [1.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
        ]
    }

    fn get_indices() -> Indices {
        vec![0, 1, 2]
    }

    fn get_draw_parameter() -> glium::DrawParameters<'b> {
        Default::default()
        // glium::DrawParameters {
        //     depth: glium::Depth {
        //         test: glium::draw_parameters::DepthTest::IfLess,
        //         write: true,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // }
    }
}
