//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
pub use rand::prelude::*;
use rand_seeder::{Seeder, SipHasher};
use rand_pcg::Pcg64;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct RandomPlugin;
impl Plugin for RandomPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Random::default());
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource)]
pub struct Random {
    seed: String,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            seed: "TANK".to_string(),
        }
    }
}

impl Random {
    pub fn new(
        seed: &str,
    ) -> Self {
        Self {
            seed: seed.to_string(),
        }
    }

    pub fn get_rng(
        &self,
    ) -> Pcg64 {
        Seeder::from(&self.seed).make_rng()
    }
}