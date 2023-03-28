use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod debug;
mod player;
mod camera;
mod testmap;
mod ui;
mod ui_interaction;
mod helpers;



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}

const INSPECT: bool = false;


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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(ui_interaction::UiInteractionPlugin)
        .add_plugin(testmap::TestMapPlugin)
        //.add_startup_system(setup_physics)
        .add_plugin(player::PlayerPlugin)

        // ----------  Always Running ----------
        .add_plugin(debug::DebugPlugin)
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