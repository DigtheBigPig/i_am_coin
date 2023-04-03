use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

//mod debug;
mod debug2;
mod player;
mod camera;
mod testmap;
mod ui;
mod ui_interaction;
mod helpers;
mod controller;
mod pause_menu;



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
                mode: bevy_window::WindowMode::Windowed,
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_startup_system(startup_setup)

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(ui_interaction::UiInteractionPlugin)
        .add_plugin(testmap::TestMapPlugin)
        //.add_startup_system(setup_physics)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(controller::ControllerPlugin)
        .add_plugin(pause_menu::PauseMenuPlugin)

        // ----------  Always Running ----------
        .add_plugin(debug2::Debug2Plugin)
        .add_plugin(helpers::HelperPlugin)
        
        
        // ----------  Menu Enter ----------
        //.add_system(main_menu::ui_setup.in_schedule(OnEnter(AppState::Menu)))
        
        // ----------  Menu Exit ----------
        //.add_system(main_menu::button_system)
        
        // ----------  InGame Enter ----------
        //.add_system(music::setup.in_schedule(OnEnter(AppState::InGame)))
        
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
    window.cursor.icon = bevy_window::CursorIcon::Move;
    window.cursor.visible = false;
    //.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
}