//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::render::mesh::*;
use bevy::render::render_resource::*;
pub use bevy::{
    window::*,
};
pub use bevy_rapier3d::prelude::*;
pub use bevy::time::common_conditions::*;
pub use std::time::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub use bevy_mod_gizmos::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// MODULES
mod ability;
pub use ability::*;
mod generics;
pub use generics::*;
mod gui;
pub use gui::*;
mod level;
pub use level::*;
mod player;
pub use player::*;
mod unit;
pub use unit::*;
mod util;
pub use util::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
const GAME_TITLE: &str = "Scratch";
const DEFAULT_RESOLUTION: (f32, f32) = (1600.0, 900.0);

pub const MAX_PLAYERS: usize = 1;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
            // Base
        app.add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: DEFAULT_RESOLUTION.into(),
                    present_mode: PresentMode::AutoNoVsync,
                    title: GAME_TITLE.into(),
                    resizable: true,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }))
            .insert_resource(Msaa::Off)
            .add_plugin(UtilsPlugin)
            .add_plugin(GenericsPlugin)

            // Crates
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            // .add_plugin(WorldInspectorPlugin::default())
            .add_plugin(GizmosPlugin)
            
            // Game
            .add_plugin(AbilityPlugin)
            .add_plugin(GuiPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(UnitPlugin)
            .add_plugin(PlayerPlugin)

            // Startup Systems
            .add_startup_system(stsys_init_game)
            
            // State Systems
            .add_system(on_enter_loading.in_schedule(OnEnter(AppState::Loading)))
            .add_system(on_update_loading.in_set(OnUpdate(AppState::Loading)))
            .add_system(on_exit_loading.in_schedule(OnExit(AppState::Loading)))

            .add_system(on_enter_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(on_exit_main_menu.in_schedule(OnExit(AppState::MainMenu)))

            .add_system(on_enter_gameplay.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(on_exit_gameplay.in_schedule(OnExit(AppState::Gameplay)));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STARTUP SYSTEMS
fn stsys_init_game(
    mut commands: Commands,
    mut fixed_time: ResMut<FixedTime>,
) {
    fixed_time.period = Duration::from_secs_f32(1.0 / 30.0);
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STATE SYSTEMS
//================================-================================-================================ 
// AppState::Loading
fn on_enter_loading() { info!("Entered Loading"); }

fn on_update_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_loader: Res<AssetLoader>,
    asset_server: Res<AssetServer>,
) {
    if !asset_loader.is_loading(&asset_server) {
        next_state.set(AppState::MainMenu);
    }
}

fn on_exit_loading() { info!("Exited Loading"); }

//================================-================================-================================ 
// AppState::MainMenu
fn on_enter_main_menu(
    mut commands: Commands,
    mut bgm_events: EventWriter<BGMEvent>,
    asset_loader: Res<AssetLoader>,
) {
    info!("Entered Main Menu");

    // BGM Test
    bgm_events.send(BGMEvent::new(asset_loader.sounds.get_handle("wind_mid"), 0.1, 0));

    // Text Binding Test
    commands.spawn(GuiTextBundle::new("", "caveat/bold", 24.0, &Color::WHITE, &asset_loader))
        .insert(TextBinding::GameTime(GameTimeValue::Elapsed));
    commands.spawn(GuiTextBundle::new("", "caveat/bold", 32.0, &Color::WHITE, &asset_loader))
        .insert(TextBinding::GameTime(GameTimeValue::TimeToPause));
}

fn on_exit_main_menu() { info!("Exited Main Menu"); }

//================================-================================-================================ 
// AppState::Gameplay
fn on_enter_gameplay() { info!("Entered Gameplay"); }
fn on_exit_gameplay() { info!("Exited Gameplay"); }