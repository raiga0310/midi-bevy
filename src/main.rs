use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod debug;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(state::AppStatePlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(state::sound::SoundPlugin)
        //.add_plugins(state::key_event::KeyBoardInputPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .run();
}
