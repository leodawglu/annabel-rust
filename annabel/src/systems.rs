/**
 * Adapted by: Leo Lu
 * For Final Rust Project
 * Spring 2023
 * Portland State University
 * 
 * Original work from:
 * Federick J Joubert "Jacques"
 * https://www.youtube.com/watch?v=iW19V3a96tY
 * https://github.com/frederickjjoubert/bevy-ball-game
 */
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::AppState;

#[derive(Component)]
pub struct MainMenuCamera;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    },
    MainMenuCamera
    ));
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::InGame {
        app_state_next_state.set(AppState::InGame);
        println!("Entered AppState::InGame");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
        app_state_next_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
        app_state_next_state.set(AppState::GameOver);
        println!("Entered AppState::GameOver");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
