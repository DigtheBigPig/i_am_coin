use bevy::prelude::*;


pub const SPEED: f32 = 35.0;
pub const ROT_SPEED: f32 = 10.0;
pub const BASE_JUMP_STRNGTH: f32 = 6.0;
pub const BASE_FLIP_STRNGTH: f32 = 20.0;
pub const MAX_JUMP_TIME_LENGTH: f32 = 1.0;


pub const SPAWN_POINT: Vec3 = Vec3::new(0.0,5.0,0.0);
pub const CAMERA_RELATIVE: Vec3 = Vec3::new(0.0,1.5,5.0);
pub const CAMERA_LOOK: Vec3 = Vec3::new(0.0,1.5,0.0);
pub const SENS_X: f32 = 0.01;
pub const SENS_Y: f32 = 0.01;