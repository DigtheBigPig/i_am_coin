use bevy::prelude::*;

pub struct UiPlugin;  

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(ui_buttons)
            .add_system(visual_button_system)
            ;
    }
}


const UI_BG: Color = Color::rgb(0.75, 0.75, 0.75);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const RED: Color = Color::rgb(1.0, 0.35, 0.35);
const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);

// components for identifying button types
#[derive(Component)]
pub struct JumpIndicator;


fn ui_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    // buttons inside a node
    commands.spawn(NodeBundle {
        style: Style {
            position: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(200.0), bottom: Val::Px(0.0) },
            size: Size::new(Val::Px(30.0), Val::Px(110.0)),
            flex_direction: FlexDirection::Column,
            // horizontally center child text
            justify_content: JustifyContent::End,
            // vertically center child text
            align_items: AlignItems::End,
            ..default()
        },
        background_color: BLACK.into(),
        ..default()
    })   
    .with_children(|parent| {
        //retry button
        parent.spawn((JumpIndicator, NodeBundle {
            style: Style {
                position: UiRect { left: Val::Px(-5.0), right: Val::Px(-5.0), top: Val::Px(-5.0), bottom: Val::Px(-5.0) },
                size: Size::new(Val::Px(20.0), Val::Px(100.0)),
                // horizontally center child text
                justify_content: JustifyContent::End,
                // vertically center child text
                align_items: AlignItems::End,
                ..default()
            },
            background_color: RED.into(),
            ..default()
        }));
    });

}

pub fn visual_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}



