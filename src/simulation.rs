use crate::color::Color;

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
                    0 => { neighbor.color.r = Self::random_addsub(self.color.r, rng) * 0.994; },
                    1 => { neighbor.color.g = Self::random_addsub(self.color.g, rng) * 0.8; },
                    2 => { neighbor.color.b = Self::random_addsub(self.color.b, rng) * 1.002; },
                    _ => ()
                }
                /*neighbor.color.r = Self::random_addsub(self.color.r, rng);
                neighbor.color.g = Self::random_addsub(self.color.g, rng);
                neighbor.color.b = Self::random_addsub(self.color.b, rng);*/
                /*neighbor.color.r = neighbor.color.r.clamp(0.0, 0.2);
                neighbor.color.g = neighbor.color.g.clamp(0.0, 0.6);*/
                if neighbor.color.b < neighbor.color.r {
                    neighbor.color.b = neighbor.color.r;
                    neighbor.color.r *= 0.995;
                }
                
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
    pub grid: Vec<Cell>,
    pub bias_map: Vec<Bias>,
    pub living_positions: Vec<usize>,
    pub count_plant: u64,
}
impl Simulation {
    pub fn seed(&mut self) {
        let size = 1024;
        let center = (size * size / 2) + (size / 2);
        /*let mut rand = oorandom::Rand64::new(0);
        for i in 0..10000 {
            let index = (self.grid.len() as f64 * rand.rand_float()) as usize;
            let r = rand.rand_float() * 1.05;
            let g = rand.rand_float() * 5.0;
            let b = rand.rand_float() * 0.5;
            
            self.grid[index].kind = Kind::PLANT;
            self.grid[index].color.set(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32);
            self.living_positions.push(index);
            self.count_plant += 1;
        }*/
        
        let r = 0.0f32;
        let g = 0.9f32;
        let b = 0.0f32;
        
        self.grid[center].kind = Kind::PLANT;
        self.grid[center].color.set(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
        self.living_positions.push(center);
        self.count_plant += 1;
    }
    
    pub fn cycle(&mut self) {
        
    }
}


pub fn new(width: usize, height: usize) -> Simulation {
    Simulation {
        grid: vec![Cell::new(); width * height],
        bias_map: vec![Bias{energy_scalar: 1.0}; width * height],
        living_positions: vec![0; 0],
        count_plant: 0,
    }
}