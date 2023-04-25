//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::{
    app::prelude::*,
    ecs::{bundle::Bundle, prelude::*},
    math::prelude::*,
    transform::components::Transform,
};

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct LookTransformPlugin;
impl Plugin for LookTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(look_transform_system);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLE
#[derive(Bundle, Clone)]
pub struct LookTransformBundle {
    pub transform: LookTransform,
    pub smoother: Smoother,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
//================================-================================-================================ 
// LookTransform
#[derive(Clone, Component, Copy, Debug)]
pub struct LookTransform {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
}

impl From<LookTransform> for Transform {
    fn from(t: LookTransform) -> Self {
        eye_look_at_target_transform(t.eye, t.target, t.up)
    }
}

impl LookTransform {
    pub fn new(
        eye: Vec3,
        target: Vec3,
        up: Vec3
    ) -> Self {
        Self {
            eye,
            target,
            up
        }
    }

    pub fn radius(
        &self
    ) -> f32 {
        (self.target - self.eye).length()
    }

    pub fn look_direction(
        &self
    ) -> Option<Vec3> {
        (self.target - self.eye).try_normalize()
    }

    pub fn set_eye_and_target(
        &mut self,
        eye: &Vec3,
        target: &Vec3,
    ) {
        self.eye = *eye;
        self.target = *target;
    }

    pub fn translate(
        &mut self,
        translation: &Vec3,
    ) {
        let target_offset = self.target - self.eye;
        self.eye += *translation;
        self.target = self.eye + target_offset;
    }

    pub fn set_translation(
        &mut self,
        translation: &Vec3,
    ) {
        let target_offset = self.target - self.eye;
        self.eye = *translation;
        self.target = self.eye + target_offset;
    }
}

fn eye_look_at_target_transform(eye: Vec3, target: Vec3, up: Vec3) -> Transform {
    let look_vector = (target - eye).normalize();
    let look_at = eye + look_vector;

    Transform::from_translation(eye).looking_at(look_at, up)
}

//================================-================================-================================ 
// Smoother
#[derive(Component, Clone)]
pub struct Smoother {
    lag_weight: f32,
    lerp_tfm: Option<LookTransform>,
    enabled: bool,
}

impl Smoother {
    pub fn new(lag_weight: f32) -> Self {
        Self {
            lag_weight,
            lerp_tfm: None,
            enabled: true,
        }
    }

    pub fn set_lag_weight(&mut self, lag_weight: f32) {
        self.lag_weight = lag_weight;
    }

    pub fn smooth_transform(&mut self, new_tfm: &LookTransform) -> LookTransform {
        debug_assert!(0.0 <= self.lag_weight);
        debug_assert!(self.lag_weight < 1.0);

        let old_lerp_tfm = self.lerp_tfm.unwrap_or(*new_tfm);

        let lead_weight = 1.0 - self.lag_weight;
        let lerp_tfm = LookTransform {
            eye: old_lerp_tfm.eye * self.lag_weight + new_tfm.eye * lead_weight,
            target: old_lerp_tfm.target * self.lag_weight + new_tfm.target * lead_weight,
            up: new_tfm.up,
        };

        self.lerp_tfm = Some(lerp_tfm);

        lerp_tfm
    }

    pub fn reset(&mut self) {
        self.lerp_tfm = None;
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn look_transform_system(
    mut cameras: Query<(&LookTransform, &mut Transform, Option<&mut Smoother>)>,
) {
    for (look_transform, mut scene_transform, smoother) in cameras.iter_mut() {
        match smoother {
            Some(mut s) if s.enabled => {
                *scene_transform = s.smooth_transform(look_transform).into()
            }
            _ => (),
        };
    }
}