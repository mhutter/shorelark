use std::f32::consts::PI;

use nalgebra::{wrap, Point2, Rotation2, Vector2};

use crate::{Food, Number};

/// View range as fraction of the world
///
/// So 0.5 means 50% of the map, 1.0 means the entire map
const FOV_RANGE: Number = 0.25;

/// How wide eyes can see
///
/// PI/2 =  90°
/// PI   = 180°
/// 2*PI = 360°
#[allow(clippy::excessive_precision)]
const FOV_ANGLE: Number = 3.92699081699; // 225 deg

/// "Resolution" of an eye
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: Number,
    fov_angle: Number,
    cells: usize,
}

impl Eye {
    /// Construct a new Eye from the given parameters
    #[must_use]
    fn new(fov_range: Number, fov_angle: Number, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);
        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn process_vision(
        &self,
        position: Point2<Number>,
        rotation: Rotation2<Number>,
        foods: &[Food],
    ) -> Vec<Number> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let distance = vec.norm();

            if distance >= self.fov_range {
                // food is outside vision range
                continue;
            }

            // get the angle of the food relative to the y axis
            let angle = Rotation2::rotation_between(&Vector2::y(), &vec).angle();
            // translate to take into account rotation of the viewer
            let angle = angle - rotation.angle();
            let angle = wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                // food is outside field of view
                continue;
            }

            // make `angle` relative to the field of view
            let angle = angle + self.fov_angle / 2.0;
            // translate to range <0,1> where 0 is the leftmost FOV border, and 1 the rightmost one
            let cell = angle / self.fov_angle;
            // translate to range <0,CELLS>
            let cell = cell * (self.cells as Number);
            // truncate to usize, cover edge case of cell = CELL
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - distance) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }

    pub fn cells(&self) -> usize {
        self.cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use test_case::test_case;

    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: Number,
        fov_angle: Number,
        x: Number,
        y: Number,
        rotation: Number,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);
            let actual_vision = eye.process_vision(
                Point2::new(self.x, self.y),
                Rotation2::new(self.rotation),
                &self.foods,
            );
            let actual_vision = actual_vision
                .into_iter()
                .map(|cell| match cell {
                    c if c >= 0.7 => '#',
                    c if c >= 0.3 => '+',
                    c if c > 0.0 => '.',
                    _ => ' ',
                })
                .collect::<String>();

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: Number, y: Number) -> Food {
        Food {
            position: Point2::new(x, y),
        }
    }

    #[test_case(1.0, "      +      ")] // Food is inside the FOV
    #[test_case(0.9, "      +      ")] // ditto
    #[test_case(0.8, "      +      ")] // ditto
    #[test_case(0.7, "      .      ")] // Food slowly disappears
    #[test_case(0.6, "      .      ")] // ditto
    #[test_case(0.5, "             ")] // Food disappeared!
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn fov_ranges(fov_range: Number, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            fov_range,
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rotation: 0.0,
            expected_vision,
        }
        .run();
    }

    #[test_case(0.00 * PI, "         +   ")] // Food is to our right
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")] // Food is to our left
    #[test_case(1.25 * PI, " +           ")]
    #[test_case(1.50 * PI, "            +")] // Food is behind us
    #[test_case(1.75 * PI, "           + ")] // (we continue to see it
    #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn rotations(rotation: Number, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            fov_range: 1.0,
            fov_angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rotation,
            expected_vision,
        }
        .run()
    }

    // Checking the X axis:
    // (you can see the bird is "flying away" from the foods)
    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   +     +   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    //
    // Checking the Y axis:
    // (you can see the bird is "flying alongside" the foods)
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "+            ")]
    fn positions(x: Number, y: Number, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
            fov_range: 1.0,
            fov_angle: FRAC_PI_2,
            rotation: 3.0 * FRAC_PI_2,
            x,
            y,
            expected_vision,
        }
        .run()
    }

    #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
    #[test_case(0.50 * PI, ".  +     +  .")]
    #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
    #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.   .")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is the widest = 8 foods
    fn fov_angles(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![
                food(0.0, 0.0),
                food(0.0, 0.33),
                food(0.0, 0.66),
                food(0.0, 1.0),
                food(1.0, 0.0),
                food(1.0, 0.33),
                food(1.0, 0.66),
                food(1.0, 1.0),
            ],
            fov_range: 1.0,
            x: 0.5,
            y: 0.5,
            rotation: 3.0 * FRAC_PI_2,
            fov_angle,
            expected_vision,
        }
        .run()
    }
}
