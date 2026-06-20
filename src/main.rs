mod world;
mod grass;
mod cell;
mod terrain;

use world::World;

fn main() {
    let mut world = World::new(10, 10);

    world.seed_grass();

    world.render();
}
