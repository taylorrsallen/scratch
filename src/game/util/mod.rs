//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// MODULES
mod app_state;
pub use app_state::*;
mod asset_loader;
pub use asset_loader::*;
mod audio_player;
pub use audio_player::*;
mod bitmask;
pub use bitmask::*;
mod camera;
pub use camera::*;
mod data;
pub use data::*;
mod defs;
pub use defs::*;
mod path;
pub use path::*;
mod random;
pub use random::*;
mod settings;
pub use settings::*;
mod vdt;
pub use vdt::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UtilsPlugin;
impl Plugin for UtilsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(AppStatePlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(AudioPlayerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(DefsPlugin)
            .add_plugin(RandomPlugin)
            .add_plugin(SettingsPlugin)
            .add_plugin(VdtPlugin);
    }
}