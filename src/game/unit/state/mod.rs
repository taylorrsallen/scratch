//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

mod property;
pub use property::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitStatePlugin;
impl Plugin for UnitStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UnitPropertyPlugin);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
/// Unit has a faction, etc.
pub struct UnitState {
    empty: u8,
    pub faction: u8,
}

impl Default for UnitState {
    fn default() -> Self {
        Self { empty: 0, faction: 0 }
    }
}

impl UnitState {
    //-------------------------------- -------------------------------- --------------------------------
    // IS
    pub fn is_neutral_or_friendly(
        &self,
        other: &Self,
    ) -> bool {
        self.faction & other.faction == other.faction
    }

    pub fn is_passable(
        &self,
        other: &Self,
    ) -> bool {
        self.faction & other.faction == other.faction // && other.state & UNIT_STATE_BUSY_MASK == 0
    }
}