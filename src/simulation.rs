use crate::color::Color;

static mut avg: f64 = 0.0;
pub const MAX_ENERGY: f32 = 10.0;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Kind {
    PLANT, HERBIVORE, EMPTY
}
use Kind::*;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub kind: Kind,
    pub color: Color,
    pub val: f32,
}
impl Cell {
    pub fn new() -> Self {
        Cell {
            kind: Kind::EMPTY,
            color: Color::new(),
            val: 0.0,
        }
    }
    
    pub fn mutate(&mut self, rng: &mut oorandom::Rand32, neighbor: &mut Cell) {
        match self.kind {
            PLANT => {
                self.val = (MAX_ENERGY / 2.0) - 1.0;
                
                neighbor.kind = PLANT;
                neighbor.color = self.color.clone();
                
                let component = rng.rand_range(0..3);
                match component {
                    0 => { neighbor.color.r = Self::random_addsub(self.color.r, rng); },
                    1 => { neighbor.color.g = Self::random_addsub(self.color.g, rng); },
                    2 => { neighbor.color.b = Self::random_addsub(self.color.b, rng); },
                    _ => ()
                }
                /*neighbor.color.r = Self::random_addsub(self.color.r, rng);
                neighbor.color.g = Self::random_addsub(self.color.g, rng);
                neighbor.color.b = Self::random_addsub(self.color.b, rng);*/
                neighbor.val = 0.0;
            },
            HERBIVORE => {
                unimplemented!()
            }
            EMPTY => panic!("Attempt to mutate an empty cell!")
        }
    }
    
    fn random_addsub(val: f32, rng: &mut oorandom::Rand32) -> f32 {
        let offset = (rng.rand_float() - 0.5) / 15.0;
        
        (val + offset).clamp(0.0, 1.0)
    }
    
    pub fn clear(&mut self) {
        self.color.clear();
        self.val = 0.0;
        self.kind = EMPTY;
    }
}

pub struct Simulation {
    pub grid: Vec<Cell>,
    pub living_positions: Vec<usize>,
    pub count_plant: u64,
}
impl Simulation {
    pub fn seed(&mut self) {
        let size = 1024;
        let center = (size * size / 2) + (size / 2);
        self.grid[center].kind = Kind::PLANT;
        self.grid[center].color.set(0.6, 0.0, 0.3);
        self.living_positions.push(center);
        self.count_plant = 1;
    }
}


pub fn new(width: usize, height: usize) -> Simulation {
    Simulation {
        grid: vec![Cell::new(); width * height],
        living_positions: vec![0; 0],
        count_plant: 0,
    }
}