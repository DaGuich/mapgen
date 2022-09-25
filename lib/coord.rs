#[derive(Debug, Copy, Clone)]
pub struct Coord {
    lon: f32,
    lat: f32,
}

impl Coord {
    pub fn new(lon: f32, lat: f32) -> Self {
        Self { lat, lon }
    }

    pub fn new_lat_lon(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }

    pub fn new_lon_lat(lon: f32, lat: f32) -> Self {
        Self { lat, lon }
    }

    pub fn lat(&self) -> f32 {
        self.lat
    }

    pub fn lat_rad(&self) -> f32 {
        self.lat.to_radians()
    }

    pub fn lon(&self) -> f32 {
        self.lon
    }

    pub fn lon_rad(&self) -> f32 {
        self.lon.to_radians()
    }
}
