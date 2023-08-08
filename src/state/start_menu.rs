use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};

use crate::state::AppState;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartMenu), start_menu_setup)
            .add_systems(Update, start_menu_system)
            .add_systems(OnExit(AppState::StartMenu), start_menu_cleanup);
    }
}

pub fn start_menu_setup() {
    println!("Start Menu setup");
}

pub fn start_menu_system(mut contexts: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
    let context = contexts.ctx_mut();
    let mut visuals = context.style().visuals.clone();
    visuals.window_fill = Color32::BLUE;
    visuals.window_stroke.width = 0.0;
    context.set_visuals(visuals);

    egui::Window::new("Start Menu").show(context, |ui| {
        if ui.button("Start Game").clicked() {
            // Start game
            app_state.set(AppState::Game);
        }
        if ui.button("Restart Game").clicked() {
            // Restart game
            app_state.set(AppState::StartMenu);
        }
        if ui.button("Settings").clicked() {
            // Settings
            // Genrate Debug menu egui window
        }
    });
}

pub fn start_menu_cleanup() {
    println!("Start Menu cleanup");
}
