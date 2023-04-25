//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::input::mouse::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct OrbitCameraPlugin;
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(evsys_receive_orbit_control_events)
            .add_event::<ControlEvent>();
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
pub enum ControlEvent {
    Orbit(Vec2),
    Zoom(f32),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLE
#[derive(Bundle)]
pub struct OrbitCameraBundle {
    orbit_cam: OrbitCam,
    #[bundle]
    look_transform: LookTransformBundle,
    transform: Transform,
}

impl OrbitCameraBundle {
    pub fn new(orbit_cam: OrbitCam, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let transform = Transform::from_translation(eye).looking_at(target, up);

        Self {
            orbit_cam,
            look_transform: LookTransformBundle {
                transform: LookTransform::new(eye, target, up),
                smoother: Smoother::new(orbit_cam.smoothing_weight),
            },
            transform,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component, Clone, Copy)]
pub struct OrbitCam {
    zoom: f32,
    smoothing_weight: f32,
}

impl Default for OrbitCam {
    fn default() -> Self {
        Self {
            zoom: 10.0,
            smoothing_weight: 0.2,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
pub fn evsys_receive_orbit_control_events(
    time: Res<Time>,
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&mut OrbitCam, &mut LookTransform, &Transform), With<Camera3d>>,
    settings: Res<Settings>,
) {
    // Can only control one camera at a time.
    let (mut orbit_cam, mut look_transform, transform) =
        if let Ok((mut orbit_cam, mut look_transform, transform)) = cameras.get_single_mut() {
            (orbit_cam, look_transform, transform)
        } else {
            return;
        };

    let mut look_angles = LookAngles::from_vector(-look_transform.look_direction().unwrap());
    let mut radius_scalar = 1.0;

    let dt = time.delta_seconds();
    for event in events.iter() {
        match event {
            ControlEvent::Orbit(delta) => {
                look_angles.add_yaw(dt * -delta.x);
                look_angles.add_pitch(dt * delta.y);
            }
            ControlEvent::Zoom(scalar) => {
                radius_scalar *= scalar;
            }
        }
    }

    orbit_cam.zoom = (radius_scalar * look_transform.radius())
        .min(settings.max_zoom)
        .max(settings.min_zoom);
    look_transform.eye = look_transform.target + orbit_cam.zoom * look_angles.unit_vector();
}