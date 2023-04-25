//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        clear_color::ClearColorConfig,
        tonemapping::*,
    },
};

mod look_angles;
pub use look_angles::*;
mod look_transform;
pub use look_transform::*;
mod orbit;
pub use orbit::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(OrbitCameraPlugin)
            .add_plugin(LookTransformPlugin);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
#[derive(Bundle)]
pub struct PlayerCameraBundle {
    name: Name,
    camera_3d_bundle: Camera3dBundle,
    bloom_settings: BloomSettings,
    player_camera: PlayerCamera,
    sound_3d_listener: Sound3dListener,
}

impl PlayerCameraBundle {
    pub fn new(player_number: u8) -> Self {
        Self {
            name: Name::new("Camera ".to_string() + &player_number.to_string()),
            camera_3d_bundle: Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..default()
                    },
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: 60.0 * 0.01745329,
                        ..default()
                    }),
                    transform: Transform::IDENTITY,
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(Color::DARK_GRAY),
                        ..default()
                    },
                    tonemapping: Tonemapping::AcesFitted,
                    dither: DebandDither::Disabled,
                    ..default()
                },
            bloom_settings: BloomSettings::default(),
            player_camera: PlayerCamera::new(player_number),
            sound_3d_listener: Sound3dListener,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct PlayerCamera {
    player_number: u8,
}

impl PlayerCamera {
    pub fn new(
        player_number: u8,
    ) -> Self {
        Self {
            player_number,
        }
    }
}