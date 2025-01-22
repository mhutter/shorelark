use rand::{rngs::ThreadRng, thread_rng};
use wasm_bindgen::prelude::wasm_bindgen;

use lib_simulation::{self as sim, Number};

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
#[allow(
    clippy::new_without_default,
    reason = "This struct only exists to interact with WASM/JS, and neither knows the concept of Default"
)]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);
        Self { rng, sim }
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);
        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness, stats.max_fitness, stats.avg_fitness,
        )
    }

    pub fn world(&self) -> World {
        self.sim.world().into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,
    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();
        Self { animals, foods }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Animal {
    pub x: Number,
    pub y: Number,
    pub rotation: Number,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Food {
    pub x: Number,
    pub y: Number,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
