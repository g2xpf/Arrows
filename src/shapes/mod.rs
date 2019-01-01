use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

glium::implement_vertex!(Vertex, position, color);

pub type Attribute = Vec<Vertex>;
pub type Index = u32;
pub type Indices = Vec<Index>;

pub trait ShapeFactory<'a, 'b, S>: ShapeFactoryInfo<'b> {
    fn new(display: &'a glium::Display) -> Self;

    fn spawn(&mut self, key: String, value: S) -> &mut S;

    fn draw<T>(&self, surface: &mut T)
    where
        T: Surface;
}

pub trait ShapeFactoryInfo<'a> {
    fn get_vertex_src() -> &'static str;
    fn get_fragment_src() -> &'static str;
    fn get_attribute() -> Attribute;
    fn get_indices() -> Indices;
    fn get_draw_parameter() -> glium::DrawParameters<'a>;
}

macro_rules! implement_shape_factory {
    ($name: ident, $ty: ty, $($id: ident), +) => {
        pub struct $name<'a> {
            program: glium::Program,
            vertex_buffer: glium::VertexBuffer<Vertex>,
            index_buffer: glium::IndexBuffer<Index>,
            draw_parameter: glium::DrawParameters<'a>,
            pub uniform: std::collections::HashMap<String, $ty>,
        }

        impl<'a, 'b> ShapeFactory<'a, 'b, $ty> for $name<'a> {
            fn new(display: &'a glium::Display) -> Self {
                $name {
                    program: glium::Program::from_source(display, Self::get_vertex_src(), Self::get_fragment_src(), None).unwrap(),
                    vertex_buffer: glium::VertexBuffer::new(display, &Self::get_attribute()).unwrap(),
                    index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &Self::get_indices()).unwrap(),
                    draw_parameter: Self::get_draw_parameter(),
                    uniform: std::collections::HashMap::new(),
                }
            }

            fn draw<T>(&self, surface: &mut T) where T: Surface {
                for (_, s) in self.uniform.iter() {
                    surface.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniform!{
                        $(
                            $id: s.$id,
                        )+
                    },  &self.draw_parameter).unwrap();
                }
            }

            fn spawn(&mut self, key: String, value: $ty) -> &mut $ty {
                self.uniform.entry(key).or_insert(value)
            }
        }
    }
}

#[macro_use]
pub mod sphere;
