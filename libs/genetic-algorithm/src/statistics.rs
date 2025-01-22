use crate::{Individual, Number};

#[derive(Debug, Clone, Copy)]
pub struct Statistics {
    pub min_fitness: Number,
    pub max_fitness: Number,
    pub avg_fitness: Number,
}

impl Statistics {
    pub(crate) fn new<I: Individual>(population: &[I]) -> Self {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as Number),
        }
    }
}
