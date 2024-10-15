use nalgebra::Point2;
use rand::{Rng, RngCore};

use crate::Number;

#[derive(Debug)]
pub struct Food {
    pub(crate) position: Point2<Number>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Food {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
