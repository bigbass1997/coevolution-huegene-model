#![allow(dead_code)]

use image::Rgba;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn new() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    
    pub fn set(&mut self, r: f32, g: f32, b: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
    
    pub fn to_u32(&self) -> u32 {
        (((self.r * 255.0) as u32) << 16) | (((self.g * 255.0) as u32) << 8) | ((self.b * 255.0) as u32)
    }
    
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from([((self.r * 255.0) as u8), ((self.g * 255.0) as u8), ((self.b * 255.0) as u8), 255])
    }
    
    pub fn clear(&mut self) {
        self.r = 0.0;
        self.g = 0.0;
        self.b = 0.0;
    }
    
    pub fn to_hue(&self) -> f32 {
        let cmax = self.r.max(self.g).max(self.b);
        let cmin = self.r.min(self.g).min(self.b);
        let dif = cmax - cmin;
        
        if dif < 0.0000001 { return 0.0; }
        
        let mut hue = 0.0;
        if cmax == self.r {
            hue =        (self.g - self.b) / dif;
        } else if cmax == self.g {
            hue = 2.0 + ((self.b - self.r) / dif);
        } else if cmax == self.b {
            hue = 4.0 + ((self.r - self.g) / dif);
        }
        hue *= 60.0;
        
        if hue < 0.0 {
            hue += 360.0;
        }
        
        hue / 360.0
    }
    
    pub fn to_abs_hue(&self) -> f32 {
        let hue = self.to_hue();
        
        if hue > 0.5 {
            return (hue - 0.5).abs() * 2.0;
        }
        
        hue * 2.0
    }
    
    pub fn to_shifted_hue(&self, shift: f32) -> f32 {
        let hue = self.to_hue() * shift;
        
        if hue > 1.0 {
            return hue - 1.0;
        } else if hue < 0.0 {
            return hue + 1.0;
        }
        
        hue
    }
}