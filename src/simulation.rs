use crate::color::Color;

pub const MAX_ENERGY: f32 = 16.0;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Kind {
    PLANT, HERBIVORE, EMPTY
}
use Kind::*;
use crate::util::InfCell;
use crate::{WIDTH, HEIGHT, WMO, HMO};

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub kind: Kind,
    pub color: Color,
    pub val: f32,
    pub compacted: bool,
}
impl Cell {
    pub fn new() -> Self {
        Cell {
            kind: Kind::EMPTY,
            color: Color::default(),
            val: 0.0,
            compacted: false,
        }
    }
    
    /// Consumes energy to produce a mutated color, which is applied to the provided neighbor cell.
    pub fn mutate(&mut self, rng: &mut oorandom::Rand32, neighbor: &mut Cell) {
        match self.kind {
            PLANT => {
                self.val = (MAX_ENERGY / 2.0) - 1.0;
                
                neighbor.kind = PLANT;
                neighbor.color = self.color.clone();
                
                let component = rng.rand_range(0..3);
                match component {
                    0 => { neighbor.color.r = Self::random_addsub(self.color.r, rng) * 0.999; },
                    1 => { neighbor.color.g = Self::random_addsub(self.color.g, rng) * 1.0005; },
                    2 => { neighbor.color.b = Self::random_addsub(self.color.b, rng) * 0.991; },
                    _ => ()
                }
                /*neighbor.color.r = Self::random_addsub(self.color.r, rng);
                neighbor.color.g = Self::random_addsub(self.color.g, rng);
                neighbor.color.b = Self::random_addsub(self.color.b, rng);*/
                /*neighbor.color.r = neighbor.color.r.clamp(0.0, 0.2);
                neighbor.color.g = neighbor.color.g.clamp(0.0, 0.6);*/
                /*if neighbor.color.b < neighbor.color.r {
                    neighbor.color.b = neighbor.color.r;
                    neighbor.color.r *= 0.995;
                }*/
                
                neighbor.val = 0.0;
            },
            HERBIVORE => {
                unimplemented!()
            }
            EMPTY => panic!("Attempt to mutate an empty cell!")
        }
    }
    
    fn random_addsub(val: f32, rng: &mut oorandom::Rand32) -> f32 {
        let offset = (rng.rand_float() - 0.5) / 10.0;
        
        (val + offset).clamp(0.0, 1.0)
    }
    
    pub fn clear(&mut self) {
        self.color.clear();
        self.val = 0.0;
        self.kind = EMPTY;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bias {
    pub energy_scalar: f32,
}

pub struct Simulation {
    pub grid: InfCell<Vec<Cell>>,
    pub bias_map: Vec<Bias>,
    pub living_positions: Vec<usize>,
}
impl Simulation {
    pub fn seed_center(&mut self, mut color: Color) {
        let center = (WIDTH * HEIGHT / 2) + (WIDTH / 2);
        
        let grid = self.grid.get_mut();
        grid[center].kind = Kind::PLANT;
        grid[center].color.set_color(color.clamp(0.0, 1.0));
        self.living_positions.push(center);
    }
    
    pub fn seed_random(&mut self, rng: &mut oorandom::Rand32, r_bias: f32, g_bias: f32, b_bias: f32, count: usize) {
        let grid = self.grid.get_mut();
        for _ in 0..count {
            let index = (grid.len() as f32 * rng.rand_float()) as usize;
            let r = rng.rand_float() * r_bias;
            let g = rng.rand_float() * g_bias;
            let b = rng.rand_float() * b_bias;
            
            grid[index].kind = Kind::PLANT;
            grid[index].color.set_rgb(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32);
            self.living_positions.push(index);
        }
    }
    
    pub fn cycle(&mut self, frame_buf: &mut [u32], rng: &mut oorandom::Rand32) {
        for vec_index in 0..self.living_positions.len() {
            let cell_index = self.living_positions[vec_index];
            let mut c = &mut self.grid.get_mut()[cell_index];
            match c.kind {
                PLANT => {
                    if c.compacted { continue }
                    
                    if c.val < MAX_ENERGY {
                        c.val += 8.00000000001 + (rng.rand_float() * 10.0);
                        c.val -= (rng.rand_float() * 2.0) * (5.0 - c.color.to_hue()).log2().abs();
                        //c.val += rng.rand_float() * 2.0 * sim.bias_map[*i].energy_scalar;
                    }
                    if c.val >= MAX_ENERGY {
                        let dir = rng.rand_range(0..4);
                        let pos = neighbor_pos(cell_index, dir);
                        let neighbor_ref = &mut self.grid.get_mut()[pos];
                        if neighbor_ref.kind == EMPTY {
                            c.mutate(rng, neighbor_ref);
                            self.living_positions.push(pos);
                        } else {
                            c.compacted = true;
                            for dir in 0..4 {
                                let pos = neighbor_pos(cell_index, dir);
                                if pos >= WIDTH * HEIGHT {
                                    println!("cell_index: {}, dir: {}, pos: {}", cell_index, dir, pos);
                                }
                                let neighbor_ref = &mut self.grid.get_mut()[pos];
                                if neighbor_ref.kind == EMPTY {
                                    c.compacted = false;
                                    break;
                                }
                            }
                        }
                    }
                },
                HERBIVORE => {
                    unimplemented!()
                }
                EMPTY => ()
            }
            
            frame_buf[cell_index] = c.color.to_u32();
        }
        
        fn neighbor_pos(pos: usize, dir: u32) -> usize {
            let x = pos % WIDTH;
            let y = pos / WIDTH;
            match dir {
                0 => { // LEFT
                    if x == 0 {
                        pos + WMO
                    } else {
                        pos - 1
                    }
                },
                1 => { // RIGHT
                    if x == WMO {
                        pos - WMO
                    } else {
                        pos + 1
                    }
                },
                2 => { // UP
                    if y == 0 {
                        pos + (WIDTH * HMO)
                    } else {
                        pos - WIDTH
                    }
                },
                3 => { // DOWN
                    if y == HMO {
                        pos - (WIDTH * HMO)
                    } else {
                        pos + WIDTH
                    }
                }
                _ => pos
            }
        }
    }
}


pub fn new(width: usize, height: usize) -> Simulation {
    Simulation {
        grid: InfCell::new(vec![Cell::new(); width * height]),
        bias_map: vec![Bias{energy_scalar: 1.0}; width * height],
        living_positions: vec![0; 0],
    }
}