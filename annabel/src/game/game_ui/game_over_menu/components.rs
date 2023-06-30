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
pub struct GameOverMenu {}

#[derive(Component)]
pub struct FinalScoreText {}

// #[derive(Component)]
// pub struct RestartButton {}

// #[derive(Component)]
// pub struct MainMenuButton {}

#[derive(Component)]
pub struct QuitButton {}
