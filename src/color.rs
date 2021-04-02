#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub fn new() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0
        }
    }
    
    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
    
    pub fn to_u32(&self) -> u32 {
        let mut col = 0u32 << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32;
        
        if col > 0 {
            //println!("{:#010x}", col);
        }
        col |= 0x00000000;
        
        col
    }
}


pub fn from_f64_rgb(r: f64, g: f64, b: f64) -> u32 {
    (((r * 255.0) as u32) << 16) | (((g * 255.0) as u32) << 8) | ((b * 255.0) as u32)
}

pub fn from_f64_rgb_gray(v: f64) -> u32 {
    from_f64_rgb(v, v, v)
}