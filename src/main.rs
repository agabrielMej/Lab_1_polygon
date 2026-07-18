mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::draw_filled_polygon;

use raylib::prelude::*;

fn main() {

    let mut fb = Framebuffer::new(800, 600);

    let polygon3 = vec![
        (377, 249),
        (411, 197),
        (436, 249),
    ];

    draw_filled_polygon(
        &mut fb,
        &polygon3,
        Color::RED,
    );

    fb.export("out.bmp");

    println!("Imagen generada.");
}