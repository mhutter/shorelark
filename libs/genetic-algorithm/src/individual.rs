use crate::{Chromosome, Number};

/// An Individual in a Population
pub trait Individual {
    /// Create an individual from the given genomes
    fn create(chromosome: Chromosome) -> Self;

    /// Determine an individual's fitness
    fn fitness(&self) -> Number;

    /// Return the encoded genetic information of the individual
    fn chromosome(&self) -> &Chromosome;
}

#[cfg(test)]
#[derive(Clone, PartialEq)]
pub enum TestIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: Number },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: Number) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            TestIndividual::WithChromosome { chromosome } => chromosome,
            TestIndividual::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness")
            }
        }
    }

    fn fitness(&self) -> Number {
        match self {
            TestIndividual::WithChromosome { chromosome } => chromosome.iter().sum(),
            TestIndividual::WithFitness { fitness } => *fitness,
        }
    }
}

#[cfg(test)]
impl std::fmt::Debug for TestIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestIndividual::WithChromosome { chromosome } => f
                .debug_struct("WithChromosome")
                .field("chromosome", &chromosome.as_ref())
                .finish(),
            TestIndividual::WithFitness { fitness } => f
                .debug_struct("WithFitness")
                .field("fitness", fitness)
                .finish(),
        }
    }
}
