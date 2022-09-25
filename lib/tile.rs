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
        let pi = std::f64::consts::PI;
        let n: f64 = 2f64.powi(zoom);
        println!("Z{:<2} -> n={n}", zoom);

        let lat_rad = coord.lat().to_radians() as f64;
        let lon_rad = coord.lon().to_radians() as f64;

        let x = (((lon_rad + pi) / (2f64 * pi)) * n).floor() as i32;
        let y = {
            let asinh = lat_rad.tan().asinh() / pi;
            (1f64 - asinh) * (n / 2f64)
        }
        .floor() as i32;

        Self::new(x, y, zoom)
    }

    pub fn contains(&self, coord: &Coord) -> bool {
        let (mpx, mpy) = self.map_pixel(coord);
        let x_bounds = (0f64..256f64).contains(&mpx);
        let y_bounds = (0f64..256f64).contains(&mpy);
        x_bounds && y_bounds
    }

    pub fn origin_pixel(&self) -> (f64, f64) {
        let x: f64 = self.x as f64 * 256f64;
        let y: f64 = self.y as f64 * 256f64;
        (x, y)
    }

    pub fn pixel(&self, coord: &Coord) -> (f64, f64) {
        let (x, y) = self.map_pixel(coord);
        let (x_orig, y_orig) = self.origin_pixel();
        (x - x_orig, y - y_orig)
    }

    pub fn map_pixel(&self, coord: &Coord) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let lat_rad = coord.lat().to_radians() as f64;
        let lon_rad = coord.lon().to_radians() as f64;
        let n = 2f64.powi(self.zoom);
        let common = (256f64 / (2f64 * pi)) * n;
        let x = common * (lon_rad + pi);
        let y = common * (pi - (pi / 4f64 + lat_rad / 2f64).tan().ln());
        (x, y)
    }
}
