use raylib::prelude::Color;

use crate::framebuffer::Framebuffer;
use crate::line::draw_line;

pub fn draw_polygon(
    fb: &mut Framebuffer,
    vertices: &[(i32, i32)],
) {
    if vertices.len() < 2 {
        return;
    }

    for i in 0..vertices.len() {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % vertices.len()];

        draw_line(fb, x0, y0, x1, y1);
    }
}

pub fn fill_polygon(
    fb: &mut Framebuffer,
    vertices: &[(i32, i32)],
    color: Color,
) {
    let min_y = vertices.iter().map(|v| v.1).min().unwrap();
    let max_y = vertices.iter().map(|v| v.1).max().unwrap();

    for y in min_y..=max_y {

        let mut intersections = Vec::<i32>::new();

        for i in 0..vertices.len() {

            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % vertices.len()];

            if (y1 <= y && y2 > y) ||
               (y2 <= y && y1 > y)
            {
                let x = x1 + ((y - y1) * (x2 - x1)) / (y2 - y1);

                intersections.push(x);
            }
        }

        intersections.sort();

        let mut i = 0;

        while i + 1 < intersections.len() {

            for x in intersections[i]..=intersections[i + 1] {
                fb.point_color(x, y, color);
            }

            i += 2;
        }
    }
}

pub fn draw_filled_polygon(
    fb: &mut Framebuffer,
    vertices: &[(i32, i32)],
    fill_color: Color,
) {
    fill_polygon(
        fb,
        vertices,
        fill_color,
    );

    fb.set_current_color(Color::WHITE);

    draw_polygon(
        fb,
        vertices,
    );
}

pub fn draw_polygon_with_hole(
    fb: &mut Framebuffer,
    outer: &[(i32, i32)],
    hole: &[(i32, i32)],
    fill_color: Color,
) {
    // Rellenar el polígono exterior
    fill_polygon(
        fb,
        outer,
        fill_color,
    );

    // "Borrar" el agujero pintándolo del color del fondo
    fill_polygon(
        fb,
        hole,
        Color::BLACK,
    );

    // Dibujar los bordes
    fb.set_current_color(Color::WHITE);

    draw_polygon(
        fb,
        outer,
    );

    draw_polygon(
        fb,
        hole,
    );
}