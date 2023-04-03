use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

pub struct PauseMenuPlugin;  

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<PauseState>()
            .add_startup_system(menu_setup)
            .add_system(pause_menu)
            .add_system(menu)
            ;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}

#[derive(Component)]
pub struct MenuItem;

// Buttons
#[derive(Component)]
pub struct ContinueButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const UI_BG: Color = Color::rgb(0.75, 0.75, 0.75);
const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);

fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // menu node
    commands.spawn((MenuItem, NodeBundle {
        visibility: Visibility::Hidden,
        style: Style {
            position: UiRect { left: Val::Px(200.0), right: Val::Px(0.0), top: Val::Px(200.0), bottom: Val::Px(0.0) },
            size: Size::new(Val::Px(500.0), Val::Px(500.0)),
            flex_direction: FlexDirection::Row,
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: BLACK.into(),
        ..default()
    }))   
    .with_children(|parent| {
        // first - button
        parent.spawn((ContinueButton, ButtonBundle {
            style: Style {
                position: UiRect { left: Val::Px(20.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
                size: Size::new(Val::Px(200.0), Val::Px(100.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        }))
        //add text to button
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Continue",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });
}


fn pause_menu(
    time: ResMut<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    pause_next_state: ResMut<NextState<PauseState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let Ok(mut window) = windows.get_single_mut() else {
            return;
        };
        window.cursor.icon = bevy_window::CursorIcon::Hand;
        window.cursor.visible = true;
        pause_ctrl(time, pause_next_state);
    }
}

fn pause_ctrl(
    mut time: ResMut<Time>,
    mut pause_next_state: ResMut<NextState<PauseState>>
) {
    match time.is_paused() {
        true =>  {time.unpause(); pause_next_state.set(PauseState::Running);},
        false => {time.pause(); pause_next_state.set(PauseState::Paused);}
    }
}

fn menu(
    pause_state: Res<State<PauseState>>,
    query_menu_items: Query<&mut Visibility, With<MenuItem>>
) {

    match pause_state.0 {
        PauseState::Paused => set_menu_visible(query_menu_items),
        PauseState::Running => set_menu_invisible(query_menu_items),
    }
}

fn set_menu_visible(
    mut query_menu_items: Query<&mut Visibility, With<MenuItem>>
) {
    for mut menu_item in query_menu_items.iter_mut() {
        *menu_item = Visibility::Visible;
    }
}

fn set_menu_invisible(
    mut query_menu_items: Query<&mut Visibility, With<MenuItem>>
) {
    for mut menu_item in query_menu_items.iter_mut() {
        *menu_item = Visibility::Hidden;
    }
}