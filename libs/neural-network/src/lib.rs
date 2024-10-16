use std::iter::once;

use rand::{Rng, RngCore};

/// The "number" type used throughout the code
pub type Number = f32;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    /// Generate a new random Network with the given Topology, i.e. number of layers and number of
    /// neurons per layer.
    ///
    /// # Panics
    ///
    /// This method assumes that the topology contains more than one layer. While a Network with
    /// one layer is technically possible, it doesn't make much sense as it represents just a
    /// single neuron.
    #[must_use]
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(
            layers.len() > 1,
            "Topology does not contain more than one layer"
        );

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    #[must_use]
    pub fn propagate(&self, inputs: Vec<Number>) -> Vec<Number> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = Number> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = Number>,
    ) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        assert!(weights.next().is_none(), "got too many weights");

        Self { layers }
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: &[Number]) -> Vec<Number> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(inputs))
            .collect()
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = Number>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();
        Self { neurons }
    }
}

#[derive(Debug)]
struct Neuron {
    bias: Number,
    weights: Vec<Number>,
}

impl Neuron {
    /// Generate a Neuron with random weights & bias
    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[Number]) -> Number {
        assert_eq!(
            inputs.len(),
            self.weights.len(),
            "Number of inputs ({}) must match number of weights ({})",
            inputs.len(),
            self.weights.len()
        );

        // multiply each input with its weight, and sum up all the results
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<Number>();

        // Apply result and clamp lower values to 0.0
        (self.bias + output).max(0.0)
    }

    fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = Number>) -> Self {
        let bias = weights.next().expect("too few weights");
        let weights = (0..input_size)
            .map(|_| weights.next().expect("too few weights"))
            .collect();
        Self { bias, weights }
    }
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn random() {
        // always return the same set of values
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref(),
        );
    }

    #[test]
    fn propagate_neuron() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Ensures `.max()` (our ReLU) works:
        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

        // `0.5` and `1.0` chosen by a fair dice roll:
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );
    }

    #[test]
    fn propagate_network() {
        // Example network from Part 1
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![
                        Neuron {
                            bias: -0.3,
                            weights: vec![0.2],
                        },
                        Neuron {
                            bias: 0.0,
                            weights: vec![1.0],
                        },
                    ],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.2,
                        weights: vec![0.6, 0.5],
                    }],
                },
            ],
        };

        let output = network.propagate(vec![0.5]);
        assert_relative_eq!(output.as_slice(), [0.45].as_ref());
        let output = network.propagate(vec![0.8]);
        assert_relative_eq!(output.as_slice(), [0.6].as_ref());

        // "passthrough" network
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.0,
                        weights: vec![1.0],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.0,
                        weights: vec![1.0],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.0,
                        weights: vec![1.0],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.0,
                        weights: vec![1.0],
                    }],
                },
            ],
        };

        let output = network.propagate(vec![1.0]);
        assert_relative_eq!(output.as_slice(), [1.0].as_ref());
    }

    #[test]
    fn weights() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.1,
                        weights: vec![0.2, 0.3, 0.4],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![0.6, 0.7, 0.8],
                    }],
                },
            ],
        };

        let actual = network.weights().collect::<Vec<_>>();
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn from_weights() {
        let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_weights(layers, weights.clone());
        let actual: Vec<_> = network.weights().collect();

        assert_relative_eq!(actual.as_slice(), weights.as_slice());
    }
}
