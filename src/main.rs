mod framebuffer;
mod fill;
mod poly1;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use poly1::draw_poligono1;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::YELLOW); // relleno
    draw_poligono1(&mut framebuffer);

    framebuffer.set_current_color(Color::WHITE); // borde
    framebuffer.draw_poligono_border(&poly1::POLY1V);

    framebuffer.render_to_file("out.png");
}
