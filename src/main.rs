mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::draw_polygon_with_hole;

use raylib::prelude::*;

fn main() {

    let mut fb = Framebuffer::new(800, 600);

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

    println!("Imagen creada correctamente.");
}