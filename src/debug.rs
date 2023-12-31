use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// App-level debug state
#[derive(Resource)]
pub struct DebugState {
    // Is debug menu visible?
    pub visible: bool,
    // A general position value to play with
    pub debug_position: Vec3,
    pub camera_look: Vec3,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugState {
            visible: false,
            debug_position: Vec3::new(0.0, 0.0, 15.5),
            camera_look: Vec3::new(0.0, 2.0, 0.0),
        })
        .add_systems(Update, debug_ui)
        .add_systems(Update, debug_controls);
    }
}

fn debug_ui(mut contexts: EguiContexts, mut debug_state: ResMut<DebugState>) {
    if debug_state.visible {
        egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
            ui.heading("General");
            ui.horizontal(|ui| {
                ui.label("Position");
                ui.add(egui::DragValue::new(&mut debug_state.debug_position.x).speed(0.1));
                ui.add(egui::DragValue::new(&mut debug_state.debug_position.y).speed(0.1));
                ui.add(egui::DragValue::new(&mut debug_state.debug_position.z).speed(0.1));
            });
            ui.horizontal(|ui| {
                ui.label("Camera target");
                ui.add(egui::DragValue::new(&mut debug_state.camera_look.x).speed(0.1));
                ui.add(egui::DragValue::new(&mut debug_state.camera_look.y).speed(0.1));
                ui.add(egui::DragValue::new(&mut debug_state.camera_look.z).speed(0.1));
            });
        });
    }
}

fn debug_controls(keyboard_input: Res<Input<KeyCode>>, mut debug_state: ResMut<DebugState>) {
    if keyboard_input.pressed(KeyCode::D) && keyboard_input.just_released(KeyCode::P) {
        debug_state.visible = !debug_state.visible;
    }
}
