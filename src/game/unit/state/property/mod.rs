//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

mod health;
pub use health::*;
mod mana;
pub use mana::*;
mod stamina;
pub use stamina::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitPropertyPlugin;
impl Plugin for UnitPropertyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HealthPlugin)
            .add_plugin(StaminaPlugin)
            .add_plugin(ManaPlugin);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
#[derive(Bundle)]
pub struct BaseUnitPropertiesBundle {
    pub health: Health,
    pub mana: Mana,
    pub stamina: Stamina,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitProperty {
    Health,
    Stamina,
    Mana,
}