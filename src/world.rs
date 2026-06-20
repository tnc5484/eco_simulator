use crate::cell::Cell;
use crate::terrain::Terrain;
use crate::grass::Grass;

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {

        let cells = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| {
                        Cell {
                            terrain: Terrain::Dirt,
                            grass: None,
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }


    pub fn render(&self) {
        for row in &self.cells {

            for cell in row {

                let symbol =
                    match &cell.grass {
                        Some(g) => match g.growth {
                            1 => ',',
                            2 => '\'',
                            3 => '"',
                            4 => 'i',
                            _ => 'g',
                        },

                        None => '.',
                    };

                print!("{}", symbol);
            }

            println!();
        }
    }

    pub fn seed_grass(&mut self) {
        for y in 2..5 {
            for x in 2..8 {
                self.cells[y][x].grass = Some(
                    Grass::new()
                );
            }
        }
    }
}