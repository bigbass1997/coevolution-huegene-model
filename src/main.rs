
extern crate minifb;

mod gif;
mod color;
mod simulation;

use simulation::*;
use simulation::Kind::*;

use minifb::{Key, Window, WindowOptions, clamp, KeyRepeat};
use std::time::{Instant};
use std::mem::swap;
use image::{Frame, RgbaImage, ImageBuffer, Rgba, ImageEncoder, ColorType};
use image::buffer::PixelsMut;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const WMO: usize = WIDTH - 1;
const HMO: usize = HEIGHT - 1;

fn main() {
    let mut sim = simulation::new(WIDTH, HEIGHT);
    //let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| { panic!("{}", e); });
    //let mut encoder = gif::new();
    let mut gif_frames = Vec::<Frame>::new();
    
    let mut rng = oorandom::Rand32::new(6);
    
    sim.seed();
    
    /* MAIN LOOP */
    let mut counter = 0;
    let mut total = 0;
    let mut last_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = Instant::now().duration_since(last_time).as_secs_f64();
        println!("{:?}, {:?}", time, 1.0 / time);
        last_time = Instant::now();
        
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
                *pixel = sim.grid[i].color.to_rgba();
            }
            gif::png_encoder().encode(&image, WIDTH as u32, HEIGHT as u32, ColorType::Rgba8).unwrap();
        }
    }
    
    //encoder.encode_frames(gif_frames).unwrap();
}

fn update(sim: &mut Simulation, buf: &mut [u32], rng: &mut oorandom::Rand32, run_decay: bool) {
    let grid_len = sim.grid.len();
    let mut count_plant = 0u64;
    let positions = sim.living_positions.clone();
    
    for i in positions.iter() {
        let grid = &mut sim.grid;
        
        let mut cel = Cell::new();
        let mut c = &mut cel;
        swap(c, &mut grid[*i]);
        
        match c.kind {
            PLANT => {
                count_plant += 1;
                if c.val < simulation::MAX_ENERGY {
                    c.val += (rng.rand_float() * 2.0) * (5.0 - c.color.to_hue()).log2().abs();
                    if c.color.g > 0.5 {
                        c.val *= 1.1;
                    }
                    if c.color.r > 0.7 {
                        c.val *= 0.8;
                    }
                }
                if c.val >= simulation::MAX_ENERGY {
                    let dir = rng.rand_range(0..4);
                    
                    let x = *i % WIDTH;
                    let y = *i / WIDTH;
                    let mut pos = *i;
                    match dir {
                        0 => { // LEFT
                            if x == 0 {
                                pos += WMO;
                            } else {
                                pos -= 1;
                            }
                        },
                        1 => { // RIGHT
                            if x == WMO {
                                pos -= WMO;
                            } else {
                                pos += 1;
                            }
                        },
                        2 => { // UP
                            if y == 0 {
                                pos += WMO * HEIGHT;
                            } else {
                                pos -= WIDTH;
                            }
                        },
                        3 => { // DOWN
                            if y == HMO {
                                pos -= HMO * WIDTH;
                            } else {
                                pos += WIDTH
                            }
                        }
                        _ => ()
                    };
                    let neighbor_ref = &mut grid[pos];
                    if neighbor_ref.kind == EMPTY {
                        c.mutate(rng, neighbor_ref);
                        sim.living_positions.push(pos);
                    }
                }
                
                if run_decay && rng.rand_float() < 0.1 {
                    c.clear();
                    let mut index = 0;
                    for (k, _) in sim.living_positions.iter().enumerate() {
                        index = k;
                    }
                    sim.living_positions.remove(index);
                }
            },
            HERBIVORE => {
                unimplemented!()
            }
            EMPTY => ()
        }
        buf[*i] = c.color.to_u32();
        
        swap(c, &mut grid[*i]);
    }
    println!("plants: {} | +{}", count_plant, (count_plant as i64) - (sim.count_plant as i64));
    
    sim.count_plant = count_plant;
    
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