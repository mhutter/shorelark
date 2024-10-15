use rand::{Rng, RngCore};

use crate::{Chromosome, Number};

/// The mutation methods describes how to introduce mutations to children
pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

#[derive(Debug, Clone)]
pub struct GaussianMutation {
    /// Probability of changig a gene
    /// - 0.0 = NO genes will be touched
    /// - 1.0 = ALL genes will be touched
    chance: Number,

    /// Magnitute of the chages
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be modified by at most 3.0
    coeff: Number,
}

impl GaussianMutation {
    /// Create a new instance with the given parameters
    ///
    /// # Panics
    ///
    /// This method panics if chance is not in `0.0..=1.0`.
    #[must_use]
    pub fn new(chance: Number, coeff: Number) -> Self {
        assert!(
            (0.0..=1.0).contains(&chance),
            "chance must be in range 0.0..=1.0"
        );
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            if rng.gen_bool(self.chance.into()) {
                *gene += self.coeff * rng.gen_range(-1.0..=1.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn actual(chance: Number, coeff: Number) -> Vec<Number> {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {
        use super::*;

        fn actual(coeff: Number) -> Vec<Number> {
            super::actual(0.0, coeff)
        }

        mod and_zero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = super::actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = super::actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        use super::*;

        fn actual(coeff: Number) -> Vec<Number> {
            super::actual(0.5, coeff)
        }

        mod and_zero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = super::actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = super::actual(0.5);
                let expected = vec![1.0, 2.0, 3.034685, 3.755602, 5.13773];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_max_chance {
        use super::*;

        fn actual(coeff: Number) -> Vec<Number> {
            super::actual(1.0, coeff)
        }

        mod and_zero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = super::actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = super::actual(0.5);
                let expected = vec![0.6872406, 2.3369198, 3.409063, 4.1314244, 5.2619405];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}
