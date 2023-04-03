use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

pub struct ControllerPlugin;  

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_player)
            ;
    }
}

const SPAWN_POINT: Vec3 = Vec3::new(0.0,5.0,0.0);
const CAMERA_RELATIVE: Vec3 = Vec3::new(0.0,1.5,5.0);
const CAMERA_LOOK: Vec3 = Vec3::new(0.0,1.5,0.0);
const SPEED: f32 = 35.0;
const ROT_SPEED: f32 = 10.0;
const BASE_JUMP_STRNGTH: f32 = 6.0;
const BASE_FLIP_STRNGTH: f32 = 20.0;
const MAX_JUMP_TIME_LENGTH: f32 = 1.0;
const SENS_X: f32 = 0.01;
const SENS_Y: f32 = 0.01;

fn move_player(
    time: Res<Time>,
    mut jump_strength: ResMut<crate::player::JumpStrength>,
    mut query_player_transform: Query<(&mut Transform, &GlobalTransform), (With<crate::player::Player>, Without<crate::camera::PlayerCamera>)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ext_impulses: Query<&mut ExternalImpulse, With<crate::player::Player>>,
    mut ext_forces: Query<&mut ExternalForce, With<crate::player::Player>>,
    rapier_context: Res<RapierContext>,
    y_rotation: Res<crate::helpers::YRotation>,
    mut player_camera: Query<&mut Transform, (With<crate::camera::PlayerCamera>, Without<crate::player::Player>)>,
    //query_player_globaltransform: Query<&GlobalTransform, With<crate::player::Player>>,
    mut camera_look: ResMut<crate::camera::CameraLook>,
    //buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    pause_state: Res<State<crate::pause_menu::PauseState>>,
) {
    if pause_state.0 == crate::pause_menu::PauseState::Paused {
        return;
    }
    
    let Ok((mut player_transform, player_global_transform)) = query_player_transform.get_single_mut() else {
        return;
    };

    //let player_pos = player_transform.translation;
    let player_rot = player_transform.rotation;
    let player_forward = y_rotation.camera_dir;
    let speed = SPEED;
    let rot_speed = ROT_SPEED;
    let speed = SPEED;
    let sens_x = 0.5;
    let sens_y = 0.5;

    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = player_rot*Vec3::new(0.0, 0.0, 0.0);
        ext_force.torque = player_rot*Vec3::new(0.0, 0.0, 0.0);
    }
    

    // movement
    for ev in motion_evr.iter() {
        camera_look.0.x -= ev.delta.x*sens_x*SENS_X;
        camera_look.0.y -= ev.delta.y*sens_y*SENS_Y;
    }

    camera_look.0.y = camera_look.0.y.clamp(-1.1, 0.40);

    // rotation
    if keyboard_input.pressed(KeyCode::A) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.torque = player_rot*Vec3::new(0.0, crate::helpers::bool_posneg(y_rotation.heads)*rot_speed, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.torque = player_rot*Vec3::new(0.0, -crate::helpers::bool_posneg(y_rotation.heads)*rot_speed, 0.0);
        }
    }

    let (_, _, player_pos) = player_global_transform.to_scale_rotation_translation();
    let camera_dirx = Quat::from_axis_angle(Vec3::Y, camera_look.0.x);
    let camera_diry = Quat::from_axis_angle(Vec3::X, camera_look.0.y);

    if keyboard_input.pressed(KeyCode::W) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = camera_dirx*Vec3::new(0.0, 0.0, -speed);
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = camera_dirx*Vec3::new(0.0, 0.0, speed);
        }
    }

    // movement
    if keyboard_input.pressed(KeyCode::J) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = camera_dirx*Vec3::new(-speed, 0.0, 0.0);
        }
    }
    if keyboard_input.pressed(KeyCode::K) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = camera_dirx*Vec3::new(speed, 0.0, 0.0);
        }
    }

    match player_camera.get_single_mut() {
        Err(_) => panic!("Not one camera"),
        Ok(mut camera_pos) => *camera_pos = Transform::from_translation(player_pos + camera_dirx*camera_diry*CAMERA_RELATIVE).looking_at(player_pos+CAMERA_LOOK, Vec3::Y),
    }


    // JUMP
    if keyboard_input.pressed(KeyCode::Space) {
        jump_strength.0 += time.delta_seconds();
        jump_strength.0 = jump_strength.0.clamp(0.0, MAX_JUMP_TIME_LENGTH);
    }
    if keyboard_input.just_released(KeyCode::Space) {
        let grounded:bool = crate::player::cast_ray(rapier_context, player_pos, player_rot);
        if grounded {
            for mut ext_impulse in ext_impulses.iter_mut() {
                ext_impulse.impulse = camera_dirx*Vec3::new(0.0, jump_strength.0*BASE_JUMP_STRNGTH, 2.0*jump_strength.0*BASE_JUMP_STRNGTH);
                ext_impulse.torque_impulse = camera_dirx*Vec3::new(-jump_strength.0*BASE_FLIP_STRNGTH ,0.0,0.0);
            }
        }
        jump_strength.0 = 0.0;
    }


    // RESET
    if keyboard_input.pressed(KeyCode::R) {
        player_transform.translation = Vec3::ZERO + SPAWN_POINT;
        player_transform.rotation = Quat::IDENTITY;
    }
    

}