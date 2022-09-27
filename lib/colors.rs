use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Default for RGB {
    fn default() -> Self {
        Self::new(u8::MAX, u8::MAX, u8::MAX)
    }
}

impl From<RGB> for [u8; 3] {
    fn from(rgb: RGB) -> Self {
        [rgb.r, rgb.g, rgb.b]
    }
}

impl From<&RGB> for [u8; 3] {
    fn from(rgb: &RGB) -> Self {
        [rgb.r, rgb.g, rgb.b]
    }
}

impl From<RGB> for [u8; 4] {
    fn from(rgb: RGB) -> Self {
        RGBA::from(rgb).into()
    }
}

impl From<&RGB> for [u8; 4] {
    fn from(rgb: &RGB) -> Self {
        RGBA::from(rgb).into()
    }
}

impl std::fmt::Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX)
    }
}

impl From<RGB> for RGBA {
    fn from(rgb: RGB) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, u8::MAX)
    }
}

impl From<&RGB> for RGBA {
    fn from(rgb: &RGB) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, u8::MAX)
    }
}

impl std::fmt::Display for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = (self.a as f32) / (u8::MAX as f32);
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, a)
    }
}

impl From<RGBA> for [u8; 4] {
    fn from(rgb: RGBA) -> Self {
        [rgb.r, rgb.g, rgb.b, rgb.a]
    }
}

impl From<&RGBA> for [u8; 4] {
    fn from(rgb: &RGBA) -> Self {
        [rgb.r, rgb.g, rgb.b, rgb.a]
    }
}
