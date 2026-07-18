mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {

    let mut fb = Framebuffer::new(800, 600);

    fb.set_current_color(Color::WHITE);

    fb.point(100, 100);

    fb.export("out.bmp");

    println!("Imagen creada correctamente.");
}