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

mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;
use crate::systems::spawn_camera;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(spawn_camera.in_schedule(OnEnter(AppState::MainMenu)))
            // Systems
            .add_systems(
                (
                    interact_with_start_button, 
                    interact_with_exit_button,
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
