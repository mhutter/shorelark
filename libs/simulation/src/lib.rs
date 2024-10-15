mod animal;
mod food;
mod world;

pub use animal::*;
pub use food::*;
use nalgebra::{distance, wrap, Vector2};
use rand::{Rng, RngCore};
pub use world::*;

pub type Number = f32;

/// Our core simulation engine
pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    /// Perform a single step of the simulation
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = distance(&animal.position, &food.position());
                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * Vector2::new(0.0, animal.speed);
            animal.position.x = wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = wrap(animal.position.y, 0.0, 1.0);
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
