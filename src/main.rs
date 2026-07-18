mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::{
    draw_filled_polygon,
    draw_polygon_with_hole,
};

use raylib::prelude::*;

fn main() {

    let mut fb = Framebuffer::new(800, 600);

    // Polígono 1 (Amarillo)
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

    draw_filled_polygon(
        &mut fb,
        &polygon1,
        Color::YELLOW,
    );

    // Polígono 2 (Azul)
    let polygon2 = vec![
        (321,335),
        (288,286),
        (339,251),
        (374,302),
    ];

    draw_filled_polygon(
        &mut fb,
        &polygon2,
        Color::BLUE,
    );

    // Polígono 3 (Rojo)
    let polygon3 = vec![
        (377,249),
        (411,197),
        (436,249),
    ];

    draw_filled_polygon(
        &mut fb,
        &polygon3,
        Color::RED,
    );

    // Polígono 4 (Verde)
    let polygon4 = vec![
        (413,177),
        (448,159),
        (502,88),
        (553,53),
        (535,36),
        (676,37),
        (660,52),
        (750,145),
        (761,179),
        (672,192),
        (659,214),
        (615,214),
        (632,230),
        (580,230),
        (597,215),
        (552,214),
        (517,144),
        (466,180),
    ];

    // Agujero (Polígono 5)
    let polygon5 = vec![
        (682,175),
        (708,120),
        (735,148),
        (739,170),
    ];

    draw_polygon_with_hole(
        &mut fb,
        &polygon4,
        &polygon5,
        Color::GREEN,
    );

    fb.export("out.bmp");

    println!("Laboratorio terminado.");
}