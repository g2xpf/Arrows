use glium;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub radius: f32,
    pub position: [f32; 3],
}

glium::implement_vertex!(Sphere, radius, position);
