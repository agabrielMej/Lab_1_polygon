use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    current_color: Color,
}

impl Framebuffer {

    pub fn new(width: i32, height: i32) -> Self {
        Self {
            image: Image::gen_image_color(width, height, Color::BLACK),
            current_color: Color::WHITE,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // Dibuja un píxel usando el color actual
    pub fn point(&mut self, x: i32, y: i32) {
        self.image.draw_pixel(x, y, self.current_color);
    }

    // Dibuja un píxel con un color específico
    pub fn point_color(
        &mut self,
        x: i32,
        y: i32,
        color: Color,
    ) {
        self.image.draw_pixel(x, y, color);
    }

    // Exporta la imagen a un archivo BMP
    pub fn export(&self, filename: &str) {
        self.image.export_image(filename);
    }
}