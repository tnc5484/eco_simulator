mod world;
mod grass;
mod cell;
mod terrain;

use world::World;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut world = World::new(10, 10);

    world.seed_grass();

    for i in 0..10{

    world.render();

    println!("going to sleep");
    sleep(Duration::from_millis(10));
    println!("Waking up");

    world.tick();
    }
}
