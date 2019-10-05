use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

/// TGA format: http://www.gamers.org/dEngine/quake3/TGA.txt
#[repr(packed)]
pub struct TGAHeader {
    pub id_length: u8,
    pub color_map_type: u8,
    pub image_type: u8,
    pub color_map_origin: u16,
    pub color_map_length: u16,
    pub color_map_depth: u8,
    pub x_origin: u16,
    pub y_origin: u16,
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u8,
    pub image_descriptor: u8,
}

fn push_le(v: &mut Vec<u8>, x: u16) {
    v.push(x as u8);
    v.push((x >> 8) as u8);
}

impl TGAHeader {
    pub fn new() -> Self {
        TGAHeader {
            id_length: 0,
            color_map_type: 0,
            image_type: 0,
            color_map_origin: 0,
            color_map_length: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 0,
            width: 0,
            height: 0,
            bits_per_pixel: 0,
            image_descriptor: 0,
        }
    }
    pub fn raw(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.push(self.id_length);
        ret.push(self.color_map_type);
        ret.push(self.image_type);
        push_le(&mut ret, self.color_map_origin);
        push_le(&mut ret, self.color_map_length);
        ret.push(self.color_map_depth);
        push_le(&mut ret, self.x_origin);
        push_le(&mut ret, self.y_origin);
        push_le(&mut ret, self.width);
        push_le(&mut ret, self.height);
        ret.push(self.bits_per_pixel);
        ret.push(self.image_descriptor);
        ret
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct TGAColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone, Debug)]
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
    pub fn new(w: usize, h: usize, bpp: usize) -> Self {
        let mut data = Vec::new();
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

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn set(&mut self, x: usize, y: usize, c: &TGAColor) -> bool {
        if x >= self.w || y >= self.h {
            return false;
        }

        let idx = (x + y * self.w) * self.bytespp;
        match c {
            TGAColor::Rgb(rgb) => {
                if self.bytespp != 3 {
                    return false;
                }
                self.data[idx] = rgb.b;
                self.data[idx + 1] = rgb.g;
                self.data[idx + 2] = rgb.r;
            }
            TGAColor::Rgba(rgba) => {
                if self.bytespp != 4 {
                    return false;
                }
                self.data[idx] = rgba.b;
                self.data[idx + 1] = rgba.g;
                self.data[idx + 2] = rgba.r;
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

    pub fn write_tga_file<P: AsRef<Path>>(&self, filename: P, rle: bool) -> bool {
        let developer_area = vec![0, 0, 0, 0];
        let extension_area = vec![0, 0, 0, 0];
        let footer = vec![
            'T' as u8, 'R' as u8, 'U' as u8, 'E' as u8, 'V' as u8, 'I' as u8, 'S' as u8, 'I' as u8,
            'O' as u8, 'N' as u8, '-' as u8, 'X' as u8, 'F' as u8, 'I' as u8, 'L' as u8, 'E' as u8,
            '.' as u8, '\0' as u8,
        ];

        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename.as_ref())
            .expect("cannot open output file");

        let mut header = TGAHeader::new();
        header.bits_per_pixel = (self.bytespp << 3).try_into().unwrap();
        header.width = self.w as u16;
        header.height = self.h as u16;
        header.image_type = if rle { 10 } else { 2 };
        header.image_descriptor = 0x20; // top-left origin
        f.write_all(&header.raw()).expect("write header error");
        if rle {
            self.write_rle_data(&mut f);
        } else {
            f.write_all(&self.data).expect("write body error");
        }
        f.write_all(&developer_area)
            .expect("write developer area error");
        f.write_all(&extension_area)
            .expect("write extension area error");
        f.write_all(&footer).expect("write footer error");
        true
    }

    fn write_rle_data<W: Write>(&self, writer: &mut W) {
        let max_chunk_length = 128;
        let mut cur_byte = 0;
        loop {
            if cur_byte >= self.w * self.h {
                break;
            }
            let mut run_length = 1;
            'inner: for cl in 0..max_chunk_length - 1 {
                for t in 0..self.bytespp {
                    if cur_byte + cl >= (self.w * self.h - 1) {
                        break 'inner;
                    }
                    if self.data[(cur_byte + cl) * self.bytespp + t]
                        != self.data[(cur_byte + cl) * self.bytespp + t + self.bytespp]
                    {
                        // should be raw, or end rle
                        break 'inner;
                    }
                }
                run_length += 1;
            }
            if run_length == 1 {
                // raw
                writer.write(&[0]).unwrap();
            } else {
                // end of rle
                let rl_byte = 128 + (run_length - 1);
                writer.write(&[rl_byte as u8]).unwrap();
            }
            writer
                .write(&self.data[cur_byte * self.bytespp..(cur_byte + 1) * self.bytespp])
                .unwrap();

            cur_byte += run_length;
        }
    }
}
