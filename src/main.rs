#![allow(warnings, unused)]
use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*, PhysicsSchedule, PhysicsStepSet};

mod player;
mod helpers;
mod game_const;

use crate::game_const::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}

const INSPECT: bool = true;


fn main() {
    App::new()
        // ----------  Initial Setup ----------
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "I am Coin!",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_systems(Startup, startup_setup)
        .add_systems(Update, debugging_ctrls)

        .add_plugins(PhysicsPlugins::default())
        //.add_startup_system(setup_physics)
        .add_plugins(player::PlayerPlugin)

        // ----------  Always Running ----------
        .add_plugins(helpers::HelperPlugin)
        
        
        // ----------  Menu Enter ----------
        
        // ----------  Menu Exit ----------
        
        // ----------  InGame Enter ----------
        
        // ----------  InGame Exit ----------

        // ----------  Pause Enter ----------
        
        // ----------  Pause Exit ----------
        
        // ----------  Exit Setup ----------
        .run();
}

fn startup_setup(
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
){
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };
    window.set_maximized(true);
    window.cursor.icon = CursorIcon::Move;
    window.cursor.visible = false;
}

fn debugging_ctrls(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform: Query<(&mut Transform, &mut Position),With<crate::player::Player>>,
    colliders: Query<(&Collider, &GlobalTransform)>,

) {
    // RESET
    if keyboard_input.just_pressed(KeyCode::R) {
        for (mut transfrom, mut position) in player_transform.iter_mut() {
            transfrom.translation = Vec3::ZERO + SPAWN_POINT;
            transfrom.rotation = Quat::IDENTITY;
            position.0 = (Vec3::ZERO + SPAWN_POINT).into();
        }
        for (collider, transform) in colliders.iter() {
            println!("collider: {:?}, position: {:?}", collider, transform.translation());
        }
    }
}