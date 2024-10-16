use std::f32::consts::FRAC_PI_2;

use lib_genetic_algorithm::{
    GaussianMutation, GeneticAlgorithm, RouletteWheelSelection, UniformCrossover,
};
use nalgebra::{distance, wrap, Rotation2, Vector2};
use rand::{Rng, RngCore};

mod animal;
mod brain;
mod eye;
mod food;
mod world;

pub use animal::*;
pub use brain::*;
pub use food::*;
pub use world::*;

pub type Number = f32;

// Some tunables
const SPEED_MIN: Number = 0.001;
const SPEED_MAX: Number = 0.005;
const SPEED_ACCEL: Number = 0.2;
const ROTATION_ACCEL: Number = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

/// Our core simulation engine
pub struct Simulation {
    world: World,
    ga: GeneticAlgorithm<RouletteWheelSelection>,
    age: usize,
    generation: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.01, 0.3),
        );
        Self {
            world,
            ga,
            age: 0,
            generation: 0,
        }
    }

    /// Perform a single step of the simulation
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;
        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = distance(&animal.position, &food.position());
                if distance <= 0.01 {
                    // eat
                    animal.satiation += 1;
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
            let response = animal.brain.nn.propagate(vision);
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

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;
        self.generation += 1;

        let current_population = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect::<Vec<_>>();

        let evolved_population = self.ga.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
