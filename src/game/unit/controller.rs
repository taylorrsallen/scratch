//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitControllerPlugin;
impl Plugin for UnitControllerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
//================================-================================-================================
// AI
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum UnitAIBehaviour {
    #[default]
    None = 0x00,
    /// Roam looking for something that satisfies UnitAIGoal
    Search = 0x01,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum UnitAIGoal {
    #[default]
    None,
    Aggressive(f32),
    Protective(f32),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
/// Unit is assigned actions by the player
#[derive(Component)]
pub struct PlayerControlled;

/// Unit is assigned actions by the AI
#[derive(Default, Component)]
pub struct AIControlled {
    behaviour: u8,
}