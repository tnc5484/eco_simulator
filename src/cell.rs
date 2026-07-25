use crate::grass::Grass;
use crate::terrain::Terrain;

pub struct Cell {
    pub terrain: Terrain,
    pub grass: Option<Grass>,
}

impl Cell{
    pub fn tick(&mut self) {
        match &mut self.grass{
            Some(grass)=> grass.tick(),
            None => {}
        }
    }
}