use crate::grass::Grass;
use crate::terrain::Terrain;

pub struct Cell {
    pub terrain: Terrain,
    pub grass: Option<Grass>,
}