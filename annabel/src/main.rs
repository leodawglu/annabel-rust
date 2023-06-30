pub const TILE_SIZE: f32 = 0.1;
mod game;
mod main_menu;
mod systems;
pub mod events;

use bevy::prelude::*;

use bevy::window::PrimaryWindow;

use main_menu::MainMenuPlugin;
use game::GamePlugin;
use systems::*;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)

        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}

pub fn spawn_camera_1(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,

) {
    let window: &Window = window_query.get_single().unwrap();
    println!("{}",window.width());
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() /2.0, 0.0 ),
        ..default()
    });
}

pub fn spawn_camera_2(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}