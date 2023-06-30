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
use bevy::prelude::Component;

#[derive(Component)]
pub struct Hud {}

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component)]
pub struct HealthText {}
