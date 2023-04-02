#[allow(dead_code)]

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;


fn camera_update(
    mut player_camera: Query<&mut Transform, With<PlayerCamera>>,
    query_player_transform: Query<&GlobalTransform, With<crate::player::Player>>,
    y_rotation: Res<crate::helpers::YRotation>,
){
    let player_transform = *query_player_transform.get_single().unwrap();
    let (_, player_rot, player_pos) = player_transform.to_scale_rotation_translation();

    match player_camera.get_single_mut() {
        Err(_) => panic!("Not one camera"),
        Ok(mut camera_pos) => *camera_pos = Transform::from_translation(player_pos + y_rotation.camera_dir*CAMERA_RELATIVE).looking_at(player_pos+CAMERA_LOOK, Vec3::Y),
    }
}

const SENS_X: f32 = 1.0;
const SENS_Y: f32 = 1.0;

fn camera_move(
    mut player_camera: Query<&mut Transform, With<PlayerCamera>>,
    query_player_transform: Query<&GlobalTransform, With<crate::player::Player>>,
    mut camera_look: ResMut<CameraLook>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
){


    // movement
    if keyboard_input.pressed(KeyCode::A) {
        camera_look.0.x += SENS_X*time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::D) {
        camera_look.0.x -= SENS_X*time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::W) {
        camera_look.0.y += SENS_Y*time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera_look.0.y -= SENS_Y*time.delta_seconds();
    }

    let player_transform = *query_player_transform.get_single().unwrap();
    let (_, player_rot, player_pos) = player_transform.to_scale_rotation_translation();
    let camera_dirx = Quat::from_axis_angle(Vec3::Y, camera_look.0.x);
    let camera_diry = Quat::from_axis_angle(Vec3::X, camera_look.0.y);

    match player_camera.get_single_mut() {
        Err(_) => panic!("Not one camera"),
        Ok(mut camera_pos) => *camera_pos = Transform::from_translation(player_pos + camera_dirx*camera_diry*CAMERA_RELATIVE).looking_at(player_pos+CAMERA_LOOK, Vec3::Y),
    }
}

fn get_y_rotation(vec: Vec3, rot: f32) -> f32 {
    let new_rot = rot*vec.y.signum();
    if new_rot<0.0 {
        return new_rot+2.0*PI;
    }
    return new_rot;
}