use std::f32::consts::FRAC_PI_2;

use lib_neural_network as nn;
use nalgebra::{distance, wrap, Rotation2, Vector2};
use rand::{Rng, RngCore};

mod animal;
mod eye;
mod food;
mod world;

pub use animal::*;
pub use food::*;
pub use world::*;

pub type Number = f32;

// Some tunables
const SPEED_MIN: Number = 0.001;
const SPEED_MAX: Number = 0.005;
const SPEED_ACCEL: Number = 0.2;
const ROTATION_ACCEL: Number = FRAC_PI_2;

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
        self.process_brains();
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

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            // look
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);

            // think
            let response = animal.brain.propagate(vision);
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // Speed & rotation are relative, so 0.0 means "keep moving as you are". This is
            // crucial since the brain does not know its own speed and rotation.

            // move
            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = Rotation2::new(animal.rotation.angle() + rotation);
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
