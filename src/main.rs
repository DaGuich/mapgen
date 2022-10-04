use mapgen::colors;
use mapgen::Image;
use std::path::Path;

fn main() {
    let path = Path::new("test.png");
    let mut image = Image::new(colors::RGBA::new(255u8, 0u8, 0u8, u8::MAX));
    let lines = vec![
        ((16, 240), (240, 240), 5),
        ((240, 240), (128, 16), 5),
        ((128, 16), (16, 240), 5),
    ];
    for ((sx, sy), (dx, dy), w) in lines {
        image.draw_line((sx, sy), (dx, dy), &colors::RGB::new(0u8, 0u8, 0u8), w);
    }
    image.write(path).unwrap();
}
