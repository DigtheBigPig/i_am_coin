use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

pub struct CameraPlugin;  

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(camera_setup)
            .add_system(camera_update)
            ;
    }
}


const CAMERA_RELATIVE: Vec3 = Vec3::new(0.0,1.5,5.0);
const CAMERA_LOOK: Vec3 = Vec3::new(0.0,1.5,0.0);

#[derive(Component)]
pub struct PlayerCamera;

fn camera_setup(
    mut commands: Commands,
) {
    
    // camera
    commands.spawn((PlayerCamera, Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }));
}

fn camera_update(
    mut player_camera: Query<&mut Transform, With<PlayerCamera>>,
    query_player_transform: Query<&GlobalTransform, With<crate::player::Player>>,
    y_rotation: Res<crate::helpers::YRotation>,
){
    let player_transform = *query_player_transform.get_single().unwrap();
    let (_, player_rot, player_pos) = player_transform.to_scale_rotation_translation();
    let changed_player_rot = player_rot.to_axis_angle();
    
    //let quat: Quat = Quat::from_axis_angle(Vec3::new(0.0,1.0,0.0), y_rotation);

    match player_camera.get_single_mut() {
        Err(_) => panic!("Not one camera"),
        Ok(mut camera_pos) => *camera_pos = Transform::from_translation(player_pos + y_rotation.quat*CAMERA_RELATIVE).looking_at(player_pos+CAMERA_LOOK, Vec3::Y),
    }
}

fn get_y_rotation(vec: Vec3, rot: f32) -> f32 {
    let new_rot = rot*vec.y.signum();
    if new_rot<0.0 {
        return new_rot+2.0*PI;
    }
    return new_rot;
}