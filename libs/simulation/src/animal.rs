use eye::Eye;
use nalgebra::{Point2, Rotation2};
use rand::{Rng, RngCore};

use super::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point2<Number>,
    pub(crate) rotation: Rotation2<Number>,
    pub(crate) speed: Number,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Animal {
        let eye = Eye::default();
        let brain = nn::Network::random(
            rng,
            &[
                // Input layer
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                // Hidden layer
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                // Output layer
                // speed & rotation
                nn::LayerTopology { neurons: 2 },
            ],
        );
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> Point2<Number> {
        self.position
    }
    pub fn rotation(&self) -> Rotation2<Number> {
        self.rotation
    }
}
