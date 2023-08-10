use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    utils::HashMap,
};

use bevy_kira_audio::prelude::*;

use super::{
    game::{PianoKey, PianoKeyId, PianoKeyInputEvent},
    AppState,
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Trucks>()
            .init_resource::<Sound>()
            .add_event::<PianoKeyInputEvent>()
            .add_plugins(AudioPlugin)
            .add_audio_channel::<Trucks>()
            .add_systems(OnEnter(AppState::Game), sound_setup)
            .add_systems(Update, (sound_system, play_sound))
            .add_systems(OnExit(AppState::Game), sound_cleanup);
    }
}

const KEYBOARD_MAPPING: [(KeyCode, f32, &str); 18] = [
    (KeyCode::Q, 246.942, "B3"),
    (KeyCode::W, 277.183, "Cs4"),
    (KeyCode::A, 261.626, "C4"),
    (KeyCode::E, 311.127, "Ds4"),
    (KeyCode::S, 293.665, "D4"),
    (KeyCode::D, 329.628, "E4"),
    (KeyCode::T, 369.994, "Fs4"),
    (KeyCode::F, 349.228, "F4"),
    (KeyCode::Y, 415.305, "Gs4"),
    (KeyCode::G, 391.995, "G4"),
    (KeyCode::U, 466.164, "As4"),
    (KeyCode::H, 440.000, "A4"),
    (KeyCode::J, 493.883, "B4"),
    (KeyCode::O, 554.365, "Cs5"),
    (KeyCode::K, 523.251, "C5"),
    (KeyCode::P, 622.254, "Ds5"),
    (KeyCode::L, 587.330, "D5"),
    (KeyCode::Semicolon, 659.255, "E5"),
];

#[derive(Resource, Default)]
pub struct Trucks {
    pub trucks: HashMap<usize, Handle<bevy_kira_audio::AudioSource>>,
}

pub fn sound_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Sound setup");
    //all notes asset loading
    let mut trucks = HashMap::default();
    for (i, (_, _, note)) in KEYBOARD_MAPPING.iter().enumerate() {
        let note_resource: Handle<bevy_kira_audio::AudioSource> =
            asset_server.load(format!("{}.mp3", note).as_str());
        trucks.insert(i, note_resource);
    }
    commands.insert_resource(Trucks { trucks });
    commands.insert_resource(Sound {
        mix_trucks: Vec::new(),
    });
}

#[derive(Resource, Default)]
pub struct Sound {
    pub mix_trucks: Vec<usize>,
}

pub fn sound_system(
    mut sounds: ResMut<Sound>,
    mut key_events: EventReader<KeyboardInput>,
    key_entityies: Query<(Entity, &PianoKeyId), With<PianoKey>>,
) {
    if key_events.is_empty() {
        return;
    }

    for key in key_events.iter() {
        for (_, key_id_component) in &key_entityies {
            let PianoKeyId(key_id) = key_id_component;
            if key.key_code == Some(KEYBOARD_MAPPING[*key_id].0) {
                println!("Key event: {:?}, key id: {:?}", key, key_id);
                // Pressed => queue the sound data
                // Released => dequeue the sound data
                match key.state {
                    ButtonState::Pressed => {
                        // prohibited to add the same key twice
                        if !sounds.mix_trucks.contains(key_id) {
                            sounds.mix_trucks.push(*key_id);
                        }
                    }
                    ButtonState::Released => {
                        sounds.mix_trucks.retain(|&x| x != *key_id);
                    }
                };
            }
        }
    }
}

pub fn play_sound(
    sounds: ResMut<Sound>,
    trucks: Res<Trucks>,
    audio: ResMut<Audio>,
    mut note_events: EventReader<PianoKeyInputEvent>,
) {
    if sounds.mix_trucks.is_empty() {
        return;
    }

    // play the sound
    for event in note_events.iter() {
        let truck = trucks.trucks.get(&event.key_id).unwrap();
        let truck_handle = truck.clone_weak();
        let truck_handle = truck_handle;
        audio.play(truck_handle);
    }
}

pub fn sound_cleanup() {
    println!("Sound cleanup");
}
