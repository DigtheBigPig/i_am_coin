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
            .init_resource::<CameraLook>()
            .add_startup_system(camera_setup)
            //.add_system(camera_move)
            ;
    }
}


const CAMERA_RELATIVE: Vec3 = Vec3::new(0.0,1.5,5.0);
const CAMERA_LOOK: Vec3 = Vec3::new(0.0,1.5,0.0);

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Resource, Debug)]
pub struct CameraLook(pub Vec2);
impl Default for CameraLook {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(0.0,0.0))
    }
}

fn camera_setup(
    mut commands: Commands,
    player: Query<Entity,With<crate::player::Player>>,
    //Debug
    mut ui_data: ResMut<crate::debug2::UiData>,
) {
    /*let camera_id = commands.spawn((PlayerCamera, Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })).id();
    match player.get_single() {
        Err(_) => panic!("Not one player"),
        Ok(player_entity) => commands
                                        .entity(player_entity)
                                        .push_children(&[camera_id]),
    };*/
    
    // camera
    let camera_id = commands.spawn((PlayerCamera, Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })).id();

    ui_data.entity = Some(camera_id);
}

const SPEED: f32 = 35.0;
const SENS_X: f32 = 0.01;
const SENS_Y: f32 = 0.01;

//move with mouse
fn camera_move(
    mut player_camera: Query<&mut Transform, With<PlayerCamera>>,
    query_player_transform: Query<&GlobalTransform, With<crate::player::Player>>,
    mut camera_look: ResMut<CameraLook>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
    time: Res<Time>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    mut ext_forces: Query<&mut ExternalForce, With<crate::player::Player>>,
) {

    let speed = SPEED;
    let sens_x = 0.5;
    let sens_y = 0.5;


    // movement
    for ev in motion_evr.iter() {
        camera_look.0.x -= ev.delta.x*sens_x*SENS_X;
        camera_look.0.y -= ev.delta.y*sens_y*SENS_Y;

        //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }

    camera_look.0.y = camera_look.0.y.clamp(-1.1, 0.40);

    if keyboard_input.just_pressed(KeyCode::Escape) {
        let Ok(mut window) = windows.get_single_mut() else {
            return;
        };
        window.cursor.icon = bevy_window::CursorIcon::Hand;
        window.cursor.visible = true;
    }    
    

    let player_transform = *query_player_transform.get_single().unwrap();
    let (_, _, player_pos) = player_transform.to_scale_rotation_translation();
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

    match player_camera.get_single_mut() {
        Err(_) => panic!("Not one camera"),
        Ok(mut camera_pos) => *camera_pos = Transform::from_translation(player_pos + camera_dirx*camera_diry*CAMERA_RELATIVE).looking_at(player_pos+CAMERA_LOOK, Vec3::Y),
    }
}