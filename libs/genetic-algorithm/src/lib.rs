use rand::RngCore;

/// The "number" type used throughout the code
pub type Number = f32;

mod chromosome;
mod crossover_method;
mod individual;
mod mutation_method;
mod selection_method;
mod statistics;

pub use chromosome::*;
pub use crossover_method::*;
pub use individual::*;
pub use mutation_method::*;
pub use selection_method::*;
pub use statistics::*;

pub struct GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    /// Evolve the given population
    ///
    /// # Panics
    ///
    /// Will panic if `population` is empty.
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty(), "Cannot evolve an empty population");

        let new_population = (0..population.len())
            .map(|_| {
                // Selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                // Crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // Mutation
                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect();

        let stats = Statistics::new(population);
        (new_population, stats)
    }
}

#[cfg(test)]
mod tests {

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn individual(genes: &[Number]) -> TestIndividual {
        TestIndividual::create(genes.iter().copied().collect())
    }

    #[test]
    fn genetic_algorithm() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        // precondition: Fitness function
        let fitness = population.iter().map(|i| i.fitness()).collect::<Vec<_>>();
        assert_eq!(fitness, &[0.0, 3.0, 4.0, 7.0]);

        for _ in 0..100 {
            population = ga.evolve(&mut rng, &population);
        }

        #[allow(clippy::excessive_precision)]
        let expected_population = vec![
            individual(&[3.1328750, 1.7573259, 3.0734420]),
            individual(&[2.8900460, 1.5473680, 2.8403120]),
            individual(&[3.0280492, 1.8644657, 2.8403120]),
            individual(&[3.3283803, 1.9080465, 4.1444077]),
        ];
        assert_eq!(population, expected_population);

        // assert that the new average fitness is highher than initially
        let fitness = population.iter().map(|i| i.fitness()).collect::<Vec<_>>();
        assert_eq!(fitness, &[7.9636426, 7.277726, 7.732827, 9.380835]);
    }
}
