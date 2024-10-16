use eye::Eye;
use lib_genetic_algorithm::{Chromosome, Individual};
use nalgebra::{Point2, Rotation2};
use rand::{Rng, RngCore};

use super::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point2<Number>,
    pub(crate) rotation: Rotation2<Number>,
    pub(crate) speed: Number,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    /// Number of foods eaten by this animal
    pub(crate) satiation: usize,
}

impl Animal {
    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Animal {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self::new(eye, brain, rng)
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, rng: &mut dyn RngCore) -> Animal {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);
        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }

    pub fn position(&self) -> Point2<Number> {
        self.position
    }
    pub fn rotation(&self) -> Rotation2<Number> {
        self.rotation
    }
}

pub struct AnimalIndividual {
    fitness: Number,
    chromosome: Chromosome,
}

impl Individual for AnimalIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn fitness(&self) -> lib_genetic_algorithm::Number {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        let fitness = animal.satiation as Number;
        let chromosome = animal.as_chromosome();
        Self {
            fitness,
            chromosome,
        }
    }
    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
