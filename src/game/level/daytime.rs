//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::pbr::CascadeShadowConfigBuilder;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct DaytimePlugin;
impl Plugin for DaytimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(scsys_init_daytime.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(sys_update_daytime.in_schedule(CoreSchedule::FixedUpdate));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct DaytimeSun {
    orbit: f32,
}

impl Default for DaytimeSun {
    fn default() -> Self {
        Self {
            orbit: 0.01745329,
        }
    }
}

#[derive(Component)]
pub struct DaytimeMoon {
    orbit: f32,
}

impl Default for DaytimeMoon {
    fn default() -> Self {
        Self {
            orbit: 0.01745329,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SCHEDULE SYSTEMS
fn scsys_init_daytime(
    mut commands: Commands,
) {
    commands.insert_resource(AmbientLight {
            color: Color::rgb_u8(140, 174, 239),
            brightness: 2.5,
        });

    commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb_u8(244, 233, 155),
                illuminance: 64_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y.any_orthonormal_vector()).with_rotation(Quat::from_axis_angle(Vec3::X, 30.0)),
            cascade_shadow_config: CascadeShadowConfigBuilder {
                num_cascades: 4,
                first_cascade_far_bound: 5.0,
                maximum_distance: 100.0,
                ..default()
            }.into(),
            ..default()
        })
        .insert(DaytimeSun::default())
        .insert(Name::new("Sun"));

    commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb_u8(140, 160, 178),
                illuminance: 48_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y.any_orthonormal_vector()),
            cascade_shadow_config: CascadeShadowConfigBuilder {
                num_cascades: 4,
                first_cascade_far_bound: 5.0,
                maximum_distance: 100.0,
                ..default()
            }.into(),
            ..default()
        })
        .insert(DaytimeMoon::default())
        .insert(Name::new("Moon"));;
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_daytime(
    mut sun_query: Query<(&mut Transform, &mut DaytimeSun), Without<DaytimeMoon>>,
    mut moon_query: Query<(&mut Transform, &mut DaytimeMoon), Without<DaytimeSun>>,
    game_time: Res<GameTime>,
) {
    for (mut transform, mut sun) in sun_query.iter_mut() {
        transform.rotate_axis(Vec3::X + Vec3::Z * 0.5, sun.orbit * game_time.delta_steps() as f32 * (1.0/30.0));
    }

    for (mut transform, mut moon) in moon_query.iter_mut() {
        transform.rotate_axis(Vec3::X + Vec3::Z * 0.5, moon.orbit * game_time.delta_steps() as f32 * (1.0/30.0));
    }
}