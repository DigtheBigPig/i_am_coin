use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::inspector_options::std_options::NumberDisplay;
use bevy_inspector_egui::{prelude::*, DefaultInspectorConfigPlugin};
use bevy_pbr::PbrBundle;
use bevy_window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

#[derive(Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
struct Config {
    // `f32` uses `NumberOptions<f32>`
    #[inspector(min = 10.0, max = 70.0, display = NumberDisplay::Slider)]
    font_size: f32,
    #[inspector(min = -1.0, speed = 0.001)] // you can specify inner options for `Option<T>`
    option: Option<f32>,
    #[inspector(min = 10, max = 20)] // same for Vec<T>
    vec: Vec<u32>,
}


pub struct Debug2Plugin;

impl Plugin for Debug2Plugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) && crate::INSPECT {
            app
            //.add_plugin(WorldInspectorPlugin::new())
            //.add_plugins(DefaultPlugins)
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            // types need to be registered
            .init_resource::<UiData>()
            .register_type::<Config>()
            .register_type::<Shape>()
            .register_type::<UiData>()
            .add_system(display_events)
            //.add_startup_system(setup)
            .add_system(ui_example);
        }
    }
}



// Enums can be have `InspectorOptions` as well.
// Note that in order to switch to another enum variant, all its fields need to have [`ReflectDefault`] type data.
#[derive(Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
enum Shape {
    Box {
        size: Vec3,
    },
    Icosphere {
        #[inspector(min = 1)]
        subdivisions: usize,
        #[inspector(min = 0.1)]
        radius: f32,
    },
    Capsule {
        radius: f32,
        rings: usize,
        depth: f32,
        latitudes: usize,
        longitudes: usize,
    },
    Line(Vec2, Vec2),
    #[default]
    UnitSphere,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct UiData {
    config: Config,
    shape: Shape,
    pub entity: Option<Entity>,
}

fn setup(mut commands: Commands, mut ui_data: ResMut<UiData>) {
    let entity = commands.spawn(PbrBundle::default()).id();
    ui_data.entity = Some(entity);
}

fn ui_example(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_resource::<UiData>(world, ui);
        });
    });
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut query_player: Query<&mut crate::player::Player, With<crate::player::Player>>,
    mut head_text_query: Query<&mut Text, (With<crate::ui::HeadIndicator>, Without<crate::ui::TailIndicator>)>,
    mut tail_text_query: Query<&mut Text, (With<crate::ui::TailIndicator>, Without<crate::ui::HeadIndicator>)>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
        let Ok(mut player) = query_player.get_single_mut() else {
            return;
        };
        player.head = crate::player::PlayerSide::Sticky;

        let mut head_text = head_text_query.get_single_mut().unwrap();
        head_text.sections[0].value = format!("Heads: {:?}", player.head);

        let mut tail_text = tail_text_query.get_single_mut().unwrap();
        tail_text.sections[0].value = format!("Tails: {:?}", player.tail);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}