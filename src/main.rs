use mapgen::Image;
use mapgen::{colors, Vec2D};
use std::path::Path;

fn main() {
    let path = Path::new("test.png");
    let mut image = Image::new(colors::RGBA::new(255u8, 0u8, 0u8, u8::MAX));
    let lines = vec![
        (Vec2D::new(16, 240), Vec2D::new(240, 240), 5),
        (Vec2D::new(240, 240), Vec2D::new(128, 16), 5),
        (Vec2D::new(128, 16), Vec2D::new(16, 240), 5),
    ];
    for (from, to, _w) in lines.iter() {
        image.draw_line(
            (from.x, from.y),
            (to.x, to.y),
            &colors::RGB::new(0u8, 0u8, 0u8),
            8,
        );
    }
    for (from, to, _w) in lines.iter() {
        image.draw_line(
            (from.x, from.y),
            (to.x, to.y),
            &colors::RGB::new(255u8, 0u8, 0u8),
            2,
        );
    }
    image.write(path).unwrap();
}
