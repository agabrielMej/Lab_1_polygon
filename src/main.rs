mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::draw_filled_polygon;

use raylib::prelude::*;

fn main() {

    let mut fb = Framebuffer::new(800, 600);

    let polygon2 = vec![
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302),
    ];

    draw_filled_polygon(
        &mut fb,
        &polygon2,
        Color::BLUE,
    );

    fb.export("out.bmp");

    println!("Imagen generada.");
}