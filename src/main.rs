
extern crate minifb;

mod gif;
mod color;
mod simulation;
mod util;

use simulation::*;
use simulation::Kind::*;

use minifb::{Key, Window, WindowOptions, clamp, KeyRepeat};
use std::time::{Instant};
use std::mem::swap;
use image::{Frame, RgbaImage, ImageBuffer, Rgba, ImageEncoder, ColorType};
use crate::color::Color;

pub const WIDTH: usize = 1800;
pub const HEIGHT: usize = 1000;
pub const WMO: usize = WIDTH - 1;
pub const HMO: usize = HEIGHT - 1;

fn main() {
    let mut sim = simulation::new(WIDTH, HEIGHT);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    //let mut buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut window = Window::new("Coevolution Huegene Model", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| { panic!("{}", e); });
    //let mut encoder = gif::new();
    //let mut gif_frames = Vec::<Frame>::new();
    
    let mut rng = oorandom::Rand32::new(99);
    
    //sim.seed_center(Color::with_rgb(0.2, 0.5, 0.5));
    //sim.seed_random(&mut rng, 0.0, 2.0, 0.4, 500);
    
    /* MAIN LOOP */
    let mut started = false;
    let mut counter = 0;
    let mut total = 0;
    let mut last_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = Instant::now().duration_since(last_time).as_secs_f64();
        println!("{:?}, {:?}", time, 1.0 / time);
        last_time = Instant::now();
        
        if !started && window.is_key_released(Key::Space) {
            sim.seed_center(Color::with_rgb(0.3, 0.05, 0.6));
            started = true;
        }
        
        update(&mut sim, &mut buffer, &mut rng, window.is_key_down(Key::D));
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        if counter == 23 {
            /*let mut image: RgbaImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
            for (i, pixel) in image.pixels_mut().enumerate() {
                *pixel = sim.grid[i].color.to_rgba();
            }
            gif_frames.push(Frame::new(image));*/
            
            counter = 0;
            total += 1;
            //println!("{}", total);
        } else {
            counter += 1;
        }
        
        if window.is_key_pressed(Key::P, KeyRepeat::No) {
            let mut image: RgbaImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
            for (i, pixel) in image.pixels_mut().enumerate() {
                *pixel = sim.grid.get()[i].color.to_rgba();
            }
            gif::png_encoder().encode(&image, WIDTH as u32, HEIGHT as u32, ColorType::Rgba8).unwrap();
        }
    }
    
    //encoder.encode_frames(gif_frames).unwrap();
}

fn update(sim: &mut Simulation, buf: &mut [u32], rng: &mut oorandom::Rand32, run_decay: bool) {
    sim.cycle(buf, rng);
    
    //println!("plants: {} | +{}", count_plant, (count_plant as i64) - (sim.count_plant as i64));
    
    
    /*for (i, c) in &mut sim.cur_grid.iter().enumerate() {
        sim.cur_grid[5].kind = Kind::EMPTY;
        
        /*let r = 1.0;
        let g = 1.0;
        let b = 1.0;
        buf[i] = from_f64_rgb(r, g, b);*/
        //gif[(i * 3)    ] = (r * 255.0) as u8;
        //gif[(i * 3) + 1] = (g * 255.0) as u8;
        //gif[(i * 3) + 2] = (b * 255.0) as u8;
    }*/
}