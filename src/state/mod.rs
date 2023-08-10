use self::game::GamePlugin;
use self::start_menu::StartMenuPlugin;
use bevy::prelude::*;

pub mod game;
pub mod key_event;
pub mod sound;
pub mod start_menu;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    StartMenu,
    Game,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugins(GamePlugin)
            .add_plugins(StartMenuPlugin);
    }
}
