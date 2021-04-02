
extern crate minifb;

mod gif;
mod color;
mod simulation;

use simulation::*;
use simulation::Kind::*;

use minifb::{Key, Window, WindowOptions, clamp};
use std::time::{Instant};
use std::mem::swap;
use image::{Frame, RgbaImage, ImageBuffer, Rgba};
use image::buffer::PixelsMut;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const WMO: usize = WIDTH - 1;
const HMO: usize = HEIGHT - 1;

fn main() {
    let mut sim = simulation::new(WIDTH, HEIGHT);
    let mut buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut gifbuf: [u8; WIDTH * HEIGHT * 3] = [0; WIDTH * HEIGHT * 3];
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| { panic!("{}", e); });
    let mut encoder = gif::new();
    let mut gif_frames = Vec::new();
    
    let mut rng = oorandom::Rand32::new(4);
    
    sim.seed();
    
    /* MAIN LOOP */
    let mut counter = 0;
    let mut total = 0;
    let mut last_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = Instant::now().duration_since(last_time).as_secs_f64();
        println!("{:?}, {:?}", time, 1.0 / time);
        last_time = Instant::now();
        let mut image: RgbaImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        
        update(&mut sim, &mut buffer, image.pixels_mut(), &mut rng);
        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        if counter == 31 {
            gif_frames.push(Frame::new(image));
            
            counter = 0;
            total += 1;
            println!("{}", total);
        } else {
            counter += 1;
        }
    }
    
    //encoder.encode_frames(gif_frames).unwrap();
}

fn update(sim: &mut Simulation, buf: &mut [u32], mut gif: PixelsMut<Rgba<u8>>, rng: &mut oorandom::Rand32) {
    let grid_len = sim.grid.len();
    let mut count_plant = 0u64;
    
    for i in 0..grid_len {
        let grid = &mut sim.grid;
        
        let mut cel = Cell::new();
        let mut c = &mut cel;
        swap(c, &mut grid[i]);
        
        match c.kind {
            PLANT => {
                count_plant += 1;
                if c.val < simulation::MAX_ENERGY {
                    c.val += 1;
                }
                if c.val == simulation::MAX_ENERGY {
                    let dir = rng.rand_range(0..4);
                    let x = i % WIDTH;
                    let y = i / WIDTH;
                    let mut pos = i;
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
                    }
                }
            },
            HERBAVORE => {
                unimplemented!()
            },
            EMPTY => ()
        }
        buf[i] = c.color.to_u32();
        *gif.next().unwrap() = Rgba::from([c.color.r, c.color.g, c.color.b, 255]);
        
        swap(c, &mut grid[i]);
    }
    println!("plants: {} | +{}", count_plant, count_plant - sim.count_plant);
    
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