use mapgen::colors;
use mapgen::Image;
use std::path::Path;

fn main() {
    let path = Path::new("test.png");
    let image = Image::new(colors::RGBA::new(255u8, 0u8, 0u8, u8::MAX));
    image.write(path).unwrap();
}
