#[macro_export]
macro_rules! texture {
    ($display: expr, $image_path: expr, $image_format: expr) => {{
        use glium;
        use image;
        use std::io::Cursor;

        let image = image::load(Cursor::new(&include_bytes!($image_path)[..]), $image_format)
            .unwrap()
            .to_rgba();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        glium::texture::Texture2d::new($display, image).unwrap()
    }};
}
