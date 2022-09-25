use crate::Coord;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    x: i32,
    y: i32,
    zoom: i32,
}

impl Tile {
    pub fn new(x: i32, y: i32, zoom: i32) -> Self {
        Self { x, y, zoom }
    }

    pub fn from_coord(coord: Coord, zoom: i32) -> Self {
        let pi = std::f32::consts::PI;
        let n: f32 = 2f32.powi(zoom);
        println!("Z{:<2} -> n={n}", zoom);

        let lat_rad = coord.lat().to_radians();
        let lon_rad = coord.lon().to_radians();

        let x = (((lon_rad + pi) / (2f32 * pi)) * n).floor() as i32;
        let y = {
            let asinh = lat_rad.tan().asinh() / pi;
            (1f32 - asinh) * (n / 2f32)
        }
        .floor() as i32;

        Self::new(x, y, zoom)
    }

    pub fn origin_pixel(&self) -> (f32, f32) {
        let x: f32 = self.x as f32 * 256f32;
        let y: f32 = self.y as f32 * 256f32;
        (x, y)
    }

    pub fn pixel(&self, coord: &Coord) -> (f32, f32) {
        let (x, y) = self.map_pixel(coord);
        (x, y % 256f32)
    }

    pub fn map_pixel(&self, coord: &Coord) -> (f32, f32) {
        let pi = std::f32::consts::PI;
        let lat_rad = coord.lat().to_radians();
        let lon_rad = coord.lon().to_radians();
        let n = 2f32.powi(self.zoom);
        let common = (256f32 / (2f32 * pi)) * n;
        let x = common * (lon_rad + pi);
        let y = common * (pi - (pi / 4f32 + lat_rad / 2f32).tan().ln());
        (x, y)
    }
}
