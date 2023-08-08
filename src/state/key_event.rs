use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub struct KeyBoardInputPlugin;

#[derive(Resource)]
pub struct KeyInputState {
    pub keys: Vec<KeyCode>,
}

impl Plugin for KeyBoardInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyInputState { keys: Vec::new() })
            .add_systems(Update, print_keyboard_event_system);
    }
}

fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}
