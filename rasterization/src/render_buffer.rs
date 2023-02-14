#[derive(Debug, Clone)]
pub struct RenderBuffer {
    width: u16,
    height: u16,
    pixels: Vec<Pixel>,
}

impl RenderBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let n_bytes = usize::from(width) * usize::from(height);
        Self {
            width,
            height,
            pixels: vec![TRANSPARENT; n_bytes],
        }
    }

    pub fn white(width: u16, height: u16) -> RenderBuffer {
        let mut buffer = RenderBuffer::new(width, height);
        let n_bytes = usize::from(width) * usize::from(height);
        for i in 0..n_bytes {
            buffer.pixels[usize::from(i)] = WHITE
        }
        buffer
    }

    pub fn raw_pixel_data(&self) -> Vec<u8> {
        let n_bytes = usize::from(self.width) * usize::from(self.height) * 4;
        let mut pixel_data = vec![0u8; n_bytes];
        for i in 0..self.pixels.len() {
            pixel_data[i * 4 + 0] = self.pixels[i].r;
            pixel_data[i * 4 + 1] = self.pixels[i].g;
            pixel_data[i * 4 + 2] = self.pixels[i].b;
            pixel_data[i * 4 + 3] = self.pixels[i].a;
        }
        pixel_data
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixels
    }
}

const WHITE: Pixel = Pixel {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
const TRANSPARENT: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
