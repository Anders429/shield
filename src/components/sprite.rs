use num_enum::UnsafeFromPrimitive;
use std::mem;

#[derive(UnsafeFromPrimitive)]
#[repr(u8)]
pub(crate) enum Pixel {
    Transparent = 0,
    Color_A = 1,
    Color_B = 2,
    Color_C = 3,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl Color {
    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Palette {
    pub(crate) color_a: Color,
    pub(crate) color_b: Color,
    pub(crate) color_c: Color,
}

pub struct Sprite {
    pub(crate) rows: [u32; 16],
}

impl Sprite {
    pub(crate) fn to_argb32_pixels(&self, palette: &Palette) -> [u8; 1024] {
        let mut result = [0; 1024];

        for (i, row) in self.rows.iter().enumerate() {
            let mut copied_row = *row;
            for j in 0..16 {
                let raw_pixel = copied_row & 0b0000_0000_0000_0011;
                copied_row >>= 2;

                match unsafe { Pixel::from_unchecked(raw_pixel as u8) } {
                    Pixel::Transparent => {
                        *unsafe { result.get_unchecked_mut(i * 64 + j * 4) } = 0;
                    }
                    Pixel::Color_A => unsafe {
                        *result.get_unchecked_mut(i * 64 + j * 4) = 255;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 1) = palette.color_a.red;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 2) = palette.color_a.green;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 3) = palette.color_a.blue;
                    },
                    Pixel::Color_B => unsafe {
                        *result.get_unchecked_mut(i * 64 + j * 4) = 255;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 1) = palette.color_b.red;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 2) = palette.color_b.green;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 3) = palette.color_b.blue;
                    },
                    Pixel::Color_C => unsafe {
                        *result.get_unchecked_mut(i * 64 + j * 4) = 255;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 1) = palette.color_c.red;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 2) = palette.color_c.green;
                        *result.get_unchecked_mut(i * 64 + j * 4 + 3) = palette.color_c.blue;
                    },
                }
            }
        }

        result
    }
}

pub(crate) struct SpriteSheet<'a, const WIDTH: usize, const HEIGHT: usize> {
    pub(crate) up: [[[&'a Sprite; WIDTH]; HEIGHT]; 2],
    pub(crate) right: [[[&'a Sprite; WIDTH]; HEIGHT]; 2],
    pub(crate) down: [[[&'a Sprite; WIDTH]; HEIGHT]; 2],
    pub(crate) left: [[[&'a Sprite; WIDTH]; HEIGHT]; 2],
}
