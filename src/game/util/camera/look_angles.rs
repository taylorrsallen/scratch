//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::math::prelude::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
const PI: f32 = std::f32::consts::PI;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
#[derive(Clone, Copy, Debug, Default)]
pub struct LookAngles {
    yaw: f32,
    pitch: f32,
}

impl LookAngles {
    pub fn from_vector(v: Vec3) -> Self {
        let mut p = Self::default();
        p.set_direction(v);

        p
    }

    pub fn unit_vector(self) -> Vec3 {
        LookAngles::unit_vector_from_yaw_and_pitch(self.yaw, self.pitch)
    }

    pub fn set_direction(&mut self, v: Vec3) {
        let (yaw, pitch) = LookAngles::yaw_and_pitch_from_vector(v);
        self.set_yaw(yaw);
        self.set_pitch(pitch);
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw % (2.0 * PI);
    }

    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }

    pub fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.get_yaw() + delta);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        let up_eps = 0.01;
        self.pitch = pitch.min(PI / 2.0 - up_eps).max(-PI / 2.0 + up_eps);
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn add_pitch(&mut self, delta: f32) {
        self.set_pitch(self.get_pitch() + delta);
    }

    fn yaw_and_pitch_from_vector(v: Vec3) -> (f32, f32) {
        debug_assert_ne!(v, Vec3::ZERO);
    
        let y = Vec3::Y;
        let z = Vec3::Z;
    
        let v_xz = Vec3::new(v.x, 0.0, v.z);
    
        if v_xz == Vec3::ZERO {
            if v.dot(y) > 0.0 {
                return (0.0, PI / 2.0);
            } else {
                return (0.0, -PI / 2.0);
            }
        }
    
        let mut yaw = v_xz.angle_between(z);
        if v.x < 0.0 {
            yaw *= -1.0;
        }
    
        let mut pitch = v_xz.angle_between(v);
        if v.y < 0.0 {
            pitch *= -1.0;
        }
    
        (yaw, pitch)
    }
    
    fn unit_vector_from_yaw_and_pitch(yaw: f32, pitch: f32) -> Vec3 {
        let ray = Mat3::from_rotation_y(yaw) * Vec3::Z;
        let pitch_axis = ray.cross(Vec3::Y);
    
        Mat3::from_axis_angle(pitch_axis, pitch) * ray
    }
}
