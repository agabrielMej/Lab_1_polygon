mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::{
    draw_polygon,
    fill_polygon,
};
use raylib::prelude::*;


fn main() {

    let mut fb = Framebuffer::new(800, 600);

    fb.set_current_color(Color::WHITE);

    // Polígono 1
    let polygon1 = vec![
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ];

    fill_polygon(
        &mut fb,
        &polygon1,
        Color::YELLOW,
    );

    fb.set_current_color(Color::WHITE);

    draw_polygon(
        &mut fb,
        &polygon1,
    );

    fb.export("out.bmp");

    println!("Imagen generada correctamente.");
}