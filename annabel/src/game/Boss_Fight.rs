use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod thorns;
pub mod thorn_manager;
pub mod protective_plants;


pub use protective_plants::*;
pub use thorn_manager::*;
use crate::AppState;

#[derive(Component)]
pub struct FlowieBoss {}

#[derive(Component)]
pub struct FlowieBossFightCamera;

pub fn spawn_flowie_boss_scene(
    mut commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    //spawn_flowie_camera(&mut commands,&window_query);
    let flowie_position: Transform = Transform::from_xyz(0.0, 0.0, 0.0 );
    let thorn_fan_timer: ThornFanTimer = Default::default();
    let flowie_boss_entity: Entity = commands.spawn((SpriteBundle {
        transform: flowie_position,
        texture: asset_server.load("art/flower_demon.png"),
        ..default()
    },
        FlowieBoss {},
        ThornManager {},
        thorn_fan_timer
    )).id();
    commands.entity(flowie_boss_entity).insert(ThornFanShootAngles::default());

}

/* This code is taken directly from unofficial Bevy CheatBook */


//Spawn named camera, may need different cameras for seperate scenes
//also useful to then scale window so assets are the right size

pub fn spawn_flowie_camera(
    mut commands: Commands,
    _window_query: Query<&Window>,
) {
    println!("occurs after boss scene");
    commands.spawn((Camera2dBundle {
        projection: {
            OrthographicProjection {
                scale: 0.7,
                    ..default()
            }
        },
        ..default()
    },
    FlowieBossFightCamera
    ));
}

pub struct BossFightPlugin;

impl Plugin for BossFightPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(spawn_flowie_boss_scene.in_schedule(OnEnter(AppState::InGame)))
            .add_system(spawn_flowie_camera.in_schedule(OnEnter(AppState::InGame)).after(spawn_flowie_boss_scene));
    }
}