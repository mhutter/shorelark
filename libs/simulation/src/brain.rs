use lib_genetic_algorithm::Chromosome;
use lib_neural_network::{LayerTopology, Network};
use rand::RngCore;

use crate::eye::Eye;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, eye: &Eye) -> Brain {
        let nn = Network::from_weights(&Self::topology(eye), chromosome);
        Self { nn }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye) -> [LayerTopology; 3] {
        [
            LayerTopology {
                neurons: eye.cells(),
            },
            LayerTopology {
                neurons: eye.cells() * 2,
            },
            LayerTopology { neurons: 2 },
        ]
    }
}
