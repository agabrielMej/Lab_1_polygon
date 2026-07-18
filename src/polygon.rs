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

        let (x1, y1) =
            vertices[(i + 1) % vertices.len()];

        draw_line(
            fb,
            x0,
            y0,
            x1,
            y1,
        );
    }
}