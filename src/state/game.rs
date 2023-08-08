use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use super::{super::debug::DebugState, key_event::KeyInputState};
use crate::state::AppState;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), game_setup)
            .add_systems(OnEnter(AppState::Game), spawn_piano)
            .add_systems(Update, game_system)
            .add_systems(
                Update,
                (
                    highlight_keys,
                    spawn_music_notes,
                    animate_music_notes,
                    clear_music_notes,
                ),
            )
            .add_systems(Update, debug_sync_camera)
            .add_systems(OnExit(AppState::Game), game_cleanup);
    }
}

#[derive(Component)]
pub struct PianoKey(usize, PianoKeyType);

#[derive(Component)]
pub struct PianoKeyId(usize);

#[derive(Component, PartialEq)]
pub enum PianoKeyType {
    White,
    Black,
}

pub fn spawn_piano(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const KEY_ORDER: [i32; 12] = [0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0];
    const NUM_TOTAL_KEYS: usize = 18; // todo: reduce this to something more reasonable
    const WHITE_KEY_WIDTH: f32 = 1.0;
    const WHITE_KEY_HEIGHT: f32 = 5.5;
    const WHITE_KEY_DEPTH: f32 = 0.25;
    const BLACK_KEY_WIDTH: f32 = 0.5;
    const BLACK_KEY_HEIGHT: f32 = 3.5;
    const BLACK_KEY_DEPTH: f32 = 0.5;

    let mut white_key_offset = -5;
    for index in 0..NUM_TOTAL_KEYS {
        let key_type_index = index % 12;
        let key_type_id = KEY_ORDER[key_type_index];
        let key_index = index as f32;
        let position_x = (white_key_offset as f32) * WHITE_KEY_WIDTH;

        if key_type_id == 0 {
            println!("generating white key {}", key_index);
            white_key_offset += 1;

            commands.spawn((
                PianoKey(index, PianoKeyType::White),
                PianoKeyId(index),
                PianoKeyType::White,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(
                        WHITE_KEY_WIDTH,
                        WHITE_KEY_HEIGHT,
                        WHITE_KEY_DEPTH,
                    ))),
                    material: materials.add(Color::WHITE.into()),
                    transform: Transform::from_xyz(position_x, 0.0, 0.0),
                    ..default()
                },
            ));
        }

        if key_type_id == 1 {
            println!("generating black key {}", key_index);
            let black_position_x = position_x + WHITE_KEY_WIDTH / 2.0;

            commands.spawn((
                PianoKey(index, PianoKeyType::Black),
                PianoKeyId(index),
                PianoKeyType::Black,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(
                        BLACK_KEY_WIDTH,
                        BLACK_KEY_HEIGHT,
                        BLACK_KEY_DEPTH,
                    ))),
                    material: materials.add(Color::BLACK.into()),
                    transform: Transform::from_xyz(black_position_x, BLACK_KEY_HEIGHT / 4.0, 0.0),
                    ..default()
                },
            ));
        }
    }
}

#[derive(Component)]
pub struct PianoNote(usize);

#[derive(Component)]
pub struct PianoNoteEvent(KeyboardInput);

pub fn spawn_music_notes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<StandardMaterial>>,
    mut key_events: EventReader<KeyboardInput>,
    piano_keys: Query<(&Transform, &PianoKeyId), With<PianoKey>>,
    key_state: Res<KeyInputState>,
) {
    if key_events.is_empty() {
        return;
    }

    for key in key_events.iter() {
        println!("[SPAWN] Music note - finding key");
        if key.state == ButtonState::Pressed {}
    }
}

pub fn animate_music_notes(
    mut notes: Query<(&mut Transform, &PianoNoteEvent), With<PianoNote>>,
    time: Res<Time>,
) {
    let animation_speed = 5.0;
    let animation_delta = time.delta().as_secs_f32() * animation_speed;

    for (mut note, key_type_component) in notes.iter_mut() {
        let PianoNoteEvent(key_type) = key_type_component;
        if key_type.state == ButtonState::Pressed {
            let scale_speed = 5.0;
            let scale_delta = time.delta().as_secs_f32() * scale_speed;
            // Scale up gradually
            note.scale.y += scale_delta;
            note.translation.y += animation_delta / 3.0;
        } else {
            // Move up
            note.translation.y += animation_delta;
        }
    }
}

pub fn clear_music_notes(
    mut commands: Commands,
    notes: Query<(Entity, &Transform), With<PianoNote>>,
) {
    for (entity, note) in notes.iter() {
        if note.translation.y > 100.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn highlight_keys(
    mut key_events: EventReader<KeyboardInput>,
    key_state: Res<KeyInputState>,
    key_entityies: Query<(Entity, &PianoKeyId, &PianoKeyType), With<PianoKey>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut key_materials: Query<&mut Handle<StandardMaterial>>,
) {
    const KEBOARD_MAPPING: [KeyCode; 18] = [
        KeyCode::Q,
        KeyCode::W,
        KeyCode::A,
        KeyCode::E,
        KeyCode::S,
        KeyCode::D,
        KeyCode::T,
        KeyCode::F,
        KeyCode::Y,
        KeyCode::G,
        KeyCode::U,
        KeyCode::H,
        KeyCode::J,
        KeyCode::O,
        KeyCode::K,
        KeyCode::P,
        KeyCode::L,
        KeyCode::Semicolon,
    ];
    if key_events.is_empty() {
        return;
    }

    for key in key_events.iter() {
        for (entity, key_id_component, key_type) in &key_entityies {
            let PianoKeyId(key_id) = key_id_component;
            if key.key_code == Some(KEBOARD_MAPPING[*key_id]) {
                println!("key id: {}", key_id);

                if let Ok(handle) = key_materials.get_mut(entity) {
                    if let Some(material) = materials.get_mut(&handle) {
                        let color: Color;
                        match key.state {
                            ButtonState::Pressed => {
                                color = Color::BLUE;
                            }
                            ButtonState::Released => {
                                color = if key_type == &PianoKeyType::Black {
                                    Color::BLACK
                                } else {
                                    Color::WHITE
                                };
                            }
                        };
                        material.base_color = color;
                    }
                }
            }
        }
    }
}

pub fn game_setup(mut commands: Commands) {
    println!("Game Setup");

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(20.0, 0.0, 50.0)
                .looking_at(Vec3::new(10.0, 5.0, 0.0), Vec3::Y),

            ..Default::default()
        },
        ThirdPersonCamera,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-3.0, 0.0, 10.0),
        ..default()
    });
}

#[derive(Component)]
pub struct ThirdPersonCamera;

pub fn debug_sync_camera(
    mut cameras: Query<(&mut Transform, &ThirdPersonCamera), Without<PianoKey>>,
    debug_state: Res<DebugState>,
) {
    if let Ok((mut camera, _)) = cameras.get_single_mut() {
        camera.translation.x = debug_state.debug_position.x;
        camera.translation.y = debug_state.debug_position.y;
        camera.translation.z = debug_state.debug_position.z;

        camera.look_at(debug_state.camera_look, Vec3::Y);
    }
}

pub fn game_system() {}

pub fn game_cleanup() {
    println!("Game cleanup");
}
