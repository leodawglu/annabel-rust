/**
 * Adapted by: Leo Lu
 * For Final Rust Project
 * Spring 2023
 * Portland State University
 * 
 * Original work from:
 * Federick J Joubert "Jacques"
 * https://www.youtube.com/watch?v=iW19V3a96tY
 */
mod components;
mod styles;
mod systems;

use systems::*;

use crate::AppState;
use bevy::prelude::*;

pub struct AnnaStatsPlugin;

impl Plugin for AnnaStatsPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_display_annastats.in_schedule(OnEnter(AppState::InGame)))
            // Systems
            .add_systems((update_score_text, update_health_text).in_set(OnUpdate(AppState::InGame)))
            // OnExit Systems
            .add_system(despawn_display_annastats.in_schedule(OnExit(AppState::InGame)));
    }
}
