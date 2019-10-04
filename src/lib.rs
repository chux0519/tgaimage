/// TGA format: http://www.gamers.org/dEngine/quake3/TGA.txt
#[repr(packed)]
pub struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

#[derive(Copy, Clone)]
pub enum TGAColor {
    Rgb(TGAColorRGB),
    Rgba(TGAColorRGBA),
}

impl TGAColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        TGAColor::Rgb(TGAColorRGB { r, g, b })
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        TGAColor::Rgba(TGAColorRGBA { r, g, b, a })
    }
}

#[derive(Copy, Clone)]
pub struct TGAColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone)]
pub struct TGAColorRGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Clone)]
pub struct TGAImage {
    data: Vec<u8>,
    w: usize,
    h: usize,
    // bytes per pixel
    bytespp: usize,
}

impl TGAImage {
    pub fn new<T: Into<usize>>(w: T, h: T, bpp: T) -> Self {
        let mut data = Vec::new();
        let w = w.into();
        let h = h.into();
        let bpp = bpp.into();
        for _ in 0..w * h * bpp {
            data.push(0);
        }
        TGAImage {
            data,
            w,
            h,
            bytespp: bpp,
        }
    }

    pub fn set<T: Into<usize>>(&mut self, x: T, y: T, c: &TGAColor) -> bool {
        let x = x.into();
        let y = y.into();
        if x >= self.w || y >= self.h {
            return false;
        }

        let idx = (x + y * self.w) * self.bytespp;
        match c {
            TGAColor::Rgb(rgb) => {
                if self.bytespp != 3 {
                    return false;
                }
                self.data[idx] = rgb.r;
                self.data[idx + 1] = rgb.g;
                self.data[idx + 2] = rgb.b;
            }
            TGAColor::Rgba(rgba) => {
                if self.bytespp != 4 {
                    return false;
                }
                self.data[idx] = rgba.r;
                self.data[idx + 1] = rgba.g;
                self.data[idx + 2] = rgba.b;
                self.data[idx + 3] = rgba.a;
            }
        }
        true
    }

    pub fn get<T: Into<usize>>(&self, x: T, y: T) -> TGAColor {
        let x = x.into();
        let y = y.into();
        let idx = (x + y * self.w) * self.bytespp;
        match self.bytespp {
            3 => TGAColor::rgb(self.data[idx], self.data[idx + 1], self.data[idx + 2]),
            4 => TGAColor::rgba(
                self.data[idx],
                self.data[idx + 1],
                self.data[idx + 2],
                self.data[idx + 3],
            ),
            _ => unreachable!(),
        }
    }

    pub fn flip_horizontally(&mut self) -> bool {
        let half = self.w >> 1;
        for i in 0..half {
            for j in 0..self.h {
                let c1 = self.get(i, j);
                let c2 = self.get(self.w - 1 - i, j);
                self.set(i, j, &c2);
                self.set(self.w - 1 - i, j, &c1);
            }
        }
        true
    }

    pub fn flip_vertically(&mut self) -> bool {
        let half = self.h / 2;
        for j in 0..half {
            for i in 0..self.w {
                let c1 = self.get(i, j);
                let c2 = self.get(i, self.h - 1 - j);
                self.set(i, j, &c2);
                self.set(i, self.h - 1 - j, &c1);
            }
        }
        true
    }
}
