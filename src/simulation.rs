use crate::color::Color;

use std::ops::Range;

static mut avg: i128 = 0;

const MUT_DIV: u32 = 1;
const MUT_RANGE: Range<u32> = 0..(MUT_DIV * 2) + 1;
pub const MAX_ENERGY: u8 = 7;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Kind {
    PLANT, HERBAVORE, EMPTY
}
use Kind::*;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub kind: Kind,
    pub color: Color,
    pub val: u8,
}
impl Cell {
    pub fn new() -> Self {
        Cell {
            kind: Kind::EMPTY,
            color: Color::new(),
            val: 0,
        }
    }
    
    pub fn mutate(&mut self, rng: &mut oorandom::Rand32, neighbor: &mut Cell) {
        match self.kind {
            PLANT => {
                self.val = (MAX_ENERGY / 2) - 1;
                
                neighbor.kind = PLANT;
                neighbor.color = self.color.clone();
                neighbor.color.r = Self::random_addsub(self.color.r, rng);
                neighbor.color.g = Self::random_addsub(self.color.g, rng);
                neighbor.color.b = Self::random_addsub(self.color.b, rng);
                neighbor.val = 0;
            },
            HERBAVORE => {
                unimplemented!()
            },
            EMPTY => panic!("Attempt to mutate an empty cell!")
        }
    }
    
    fn random_addsub(val: u8, rng: &mut oorandom::Rand32) -> u8 {
        let offset = ((rng.rand_float() - 0.5) * 8.0);
        //let mut offset = rng.rand_range(MUT_RANGE) as i8;
        //offset -= MUT_DIV as i8 + 0i8;

        unsafe {
            avg += offset as i128;
            println!("{:#02} | {}", offset, avg);
        };
        
        if offset > 0.0 {
            return val.wrapping_add(offset.abs() as u8);
        } else if offset < 0.0 {
            return val.wrapping_sub(offset.abs() as u8);
        }
        
        val
    }
}

pub struct Simulation {
    pub grid: Vec<Cell>,
    pub count_plant: u64,
}
impl Simulation {
    pub fn seed(&mut self) {
        self.grid[32896].kind = Kind::PLANT;
        self.grid[32896].color.set(255, 255, 255);
        
        self.count_plant = 1;
    }
}


pub fn new(width: usize, height: usize) -> Simulation {
    Simulation {
        grid: vec![Cell::new(); width * height],
        count_plant: 0,
    }
}