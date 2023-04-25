//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::input::mouse::*;
use bevy_kira_audio::prelude::AudioReceiver;

mod input;
pub use input::*;
mod quickslots;
pub use quickslots::*;

mod marker;
pub use marker::*;

mod designator;
pub use designator::*;
mod orderer;
pub use orderer::*;

mod select;
pub use select::*;
mod selector;
pub use selector::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputPlugin)
            .add_plugin(QuickslotsPlugin)
            .add_plugin(MarkerPlugin)
            .add_plugin(DesignatePlugin)
            .add_plugin(OrdererPlugin)
            .add_plugin(SelectPlugin)
            .add_plugin(SelectorPlugin)
            .configure_sets((
                PlayerSet::Input
                    .after(CoreSet::StateTransitions)
                    .before(PlayerSet::Commands),
                PlayerSet::Commands
                    .after(PlayerSet::Input)
                    .before(PlayerSet::State),
                PlayerSet::State
                    .after(PlayerSet::Commands)
                    .before(PlayerSet::Actions),
                PlayerSet::Actions
                    .after(PlayerSet::State)
                    .before(PlayerSet::Last),
                PlayerSet::Last
                    .after(PlayerSet::Actions)
                    .before(CoreSet::Update),
            ))
            .add_startup_system(stsys_spawn_player)
            .add_systems((
                    sys_update_game_time_pause_delay,
                ).in_base_set(PlayerSet::State))
            .add_systems((
                    sys_update_player_camera,
                ).in_base_set(PlayerSet::Actions));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEM SETS
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[system_set(base)]
pub enum PlayerSet {
    /// Refresh raw input
    Input,
    /// Generate commands from raw input
    Commands,
    /// Anything that would affect command interpretation
    State,
    /// Interpret commands based on state and fire off actions
    Actions,
    /// Hide/Show cursor/marker
    Last,
}


//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
//================================-================================-================================ 
// TacticsPlayerBundle
#[derive(Bundle)]
pub struct TacticsPlayerBundle {
    player: Player,
    quickslots: Quickslots,
    designator: PlayerDesignator,
    orderer: PlayerOrderer,
    selector: PlayerSelector,
}

impl TacticsPlayerBundle {
    pub fn new(
        commands: &mut Commands,
        camera: Entity,
        asset_loader: &Res<AssetLoader>,
    ) -> Self {
        let cursor = commands.spawn(SceneBundle {
                scene: asset_loader.models.get_handle("cursor"),
                transform: Transform::IDENTITY.looking_to(Vec3::NEG_Y, Vec3::X),
                ..default()
            })
            .insert(SelectionMarker)
            .insert(Name::new("Cursor"))
            .id();

        Self {
            player: Player::new(camera),
            quickslots: Quickslots::new(
                Some(Sound2dEvent::new(asset_loader.sounds.get_handle("8bit/blip_high"), 1.0)),
                Some(Sound2dEvent::new(asset_loader.sounds.get_handle("select_02"), 1.0)),
            ),
            designator: PlayerDesignator::default(),
            orderer: PlayerOrderer::default(),
            selector: PlayerSelector::new(cursor),
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct Player {
    camera: Entity,
}

impl Player {
    pub fn new(camera: Entity) -> Self {
        Self { camera }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STARTUP SYSTEMS
fn stsys_spawn_player(
    mut commands: Commands,
    asset_loader: Res<AssetLoader>,
) {
    let tactice_camera = commands.spawn(PlayerCameraBundle::new(0))
        .insert(OrbitCameraBundle::new(OrbitCam::default(), Vec3::new(16.0, 16.0, 16.0), Vec3::new(0.0, 6.0, 0.0), Vec3::Y))
        .insert(AudioReceiver)
        .id();
    let tactics_player_bundle = TacticsPlayerBundle::new(&mut commands, tactice_camera, &asset_loader);
    let player = commands.spawn(tactics_player_bundle).id();
    spawn_quickslots_gui(player, &mut commands, &asset_loader);
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_player_camera(
    mut camera_query: Query<(&mut LookTransform, &Transform), With<Camera3d>>,
    mut control_events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    player_query: Query<&Player>,
    input_state: Res<InputState>,
    settings: Res<Settings>,
    time: Res<Time>,
) {
    for (player) in player_query.iter() {
        let (mut look_transform, transform) = if let Ok((mut look_transform, transform)) = camera_query.get_mut(player.camera) {
                (look_transform, transform)
            } else {
                continue;
            };
        let mut movement_input = Vec3::ZERO;
    
        let mut forward = transform.forward();
        forward.y = 0.0;
        forward = forward.normalize();
    
        let mut right = transform.right();
        right.y = 0.0;
        right = right.normalize();
    
        // let mut up = transform.up();
        // up = up.normalize();
    
        for (input_action, direction) in [
            (InputAction::MoveLeft, -right),
            (InputAction::MoveRight, right),
            (InputAction::MoveDown, Vec3::NEG_Y),
            (InputAction::MoveUp, Vec3::Y),
            (InputAction::MoveBack, -forward),
            (InputAction::MoveForward, forward),
        ].iter().cloned() {
            if input_state.pressed(input_action) {
                movement_input += direction;
            }
        }

        let mut speed = settings.camera_speed;
        for (input_action, multiplier) in [
            (InputAction::FastMod, settings.camera_fast_mult),
            (InputAction::SlowMod, settings.camera_slow_mult),
        ].iter().cloned() {
            if input_state.pressed(input_action) {
                speed *= multiplier;
            }
        }
    
        if let Some(movement) = movement_input.try_normalize() {
            look_transform.translate(&(movement * speed * time.delta_seconds()));
        }

        if input_state.pressed(InputAction::TertiaryAction) {
            let mut cursor_delta = Vec2::ZERO;
            for event in mouse_motion_events.iter() {
                cursor_delta += event.delta;
            }
    
            control_events.send(ControlEvent::Orbit(settings.camera_pivot_speed * cursor_delta));
        }
    
        let mut scalar = 1.0;
        for event in mouse_wheel_reader.iter() {
            // scale the event magnitude per pixel or per line
            let scroll_amount = match event.unit {
                MouseScrollUnit::Line => event.y,
                MouseScrollUnit::Pixel => event.y / 53.0,
            };
            scalar *= 1.0 - scroll_amount * settings.camera_zoom_speed;
        }
    
        control_events.send(ControlEvent::Zoom(scalar));
    }
}

fn sys_update_game_time_pause_delay(
    mut game_time: ResMut<GameTime>,
    player_unit_query: Query<Entity, With<PlayerControlled>>,
    orders_query: Query<&UnitOrderable, With<PlayerControlled>>,
    actioner_query: Query<&UnitActioner, With<PlayerControlled>>,
    mover_query: Query<&UnitMover, With<PlayerControlled>>,
) {
    // if game_time.paused() && game_time.locked() {
    //     let mut steps_to_next_free_unit = u32::MAX;
    //     for (mobility, pathing, mind) in player_unit_query.iter() {
    //         let steps_to_next_action = Unit::steps_until_next_action(&mobility, &pathing);
    //         if steps_to_next_action < steps_to_next_free_unit {
    //             steps_to_next_free_unit = steps_to_next_action;
    //         }
    //     }
    
    //     if steps_to_next_free_unit != u32::MAX {
    //         game_time.set_steps_until_pause(steps_to_next_free_unit);
    //     }
    // }
}