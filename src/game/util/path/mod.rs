//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

mod astar;
pub use astar::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        
    }
}