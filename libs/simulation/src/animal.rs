use nalgebra::{Point2, Rotation2};
use rand::{Rng, RngCore};

use crate::Number;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point2<Number>,
    pub(crate) rotation: Rotation2<Number>,
    pub(crate) speed: Number,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Animal {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> Point2<Number> {
        self.position
    }
    pub fn rotation(&self) -> Rotation2<Number> {
        self.rotation
    }
}
