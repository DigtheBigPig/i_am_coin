use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_egui::EguiPlugin;
use std::any::TypeId;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) && crate::INSPECT {
            app
            //.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(EguiPlugin)
            .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
            .add_system(inspector_ui);
        }
    }
}


fn inspector_ui(world: &mut World) {
    let egui_context = world.resource_mut::<bevy_egui::EguiContext>().ctx_mut().clone();

    egui::Window::new("UI").show(&egui_context, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            // bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}