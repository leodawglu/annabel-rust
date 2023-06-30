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

use systems::*;

use crate::AppState;
use bevy::prelude::*;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_systems(
                (
                    interact_with_quit_button,
                    update_final_score_text,
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            // // OnExit State Systems
            .add_system(update_final_score_text.in_schedule(OnExit(AppState::GameOver)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)).after(update_final_score_text));
    }
}