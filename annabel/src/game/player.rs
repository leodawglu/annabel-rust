/***
 * This file contains functions that dictate spawning player sprite,
 * controlling player movement, player animation, as well as bounding the
 * player's movement to inside the camera bounds.
 */

use bevy::prelude::*;
use super::SimulationState;
use crate::AppState;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use crate::game::physics::colliders::*;
use crate::events::GameOver;

pub const PLAYER_SPEED: f32 = 225.0;

#[derive(Component)]
pub struct Anna {}

#[derive(Resource)]
pub struct AnnaStats {
    pub health: f32,
    pub score: f32
}

impl Default for AnnaStats {
    fn default() -> Self {
        AnnaStats {
            health: 10.0,
            score: 0.0
        }
    }
}

//Registered as injury event
pub struct AnnaInjured {
    pub damage: f32
}

//Registered as death event
pub struct AnnaDeath;

pub struct UpdateAnnaScore;

pub fn update_score(
    mut score_event: EventReader<UpdateAnnaScore>,
    mut anna_stats: ResMut<AnnaStats>
) {
    for _ in score_event.iter() {
        anna_stats.score += 5.0;
        println!("ANNA's Score is now: {}", anna_stats.score)
    }

}

//not called as system in game loop, rather spawned as
//child entity of level so that positioning
//is relative to some marker
// pub fn setup_anna(
//     mut commands: &mut Commands,
//     asset_server: &Res<AssetServer>,
// ) -> Entity {
//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_xyz(0.0, -100.0, 0.0),
//             texture: asset_server.load("art/Anna_stopped_up.png"),
//             ..default()
//         },
//         Anna {},
//     )).id()
// }
pub fn setup_anna(
    mut commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {    
    let anna_entity = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            texture: asset_server.load("art/Anna_stopped_up.png"),
            ..default()
        },
        Anna {},
    )).id();

    let anna_membership: Group = Group::from_bits_truncate(0b0001);
    let anna_filter: Group = Group::from_bits_truncate(0b0110);

    let bundle = SensorCollisionBundle {
        entity: anna_entity,
        transform: Transform::from_xyz(0., 0., 0.),
        membership: anna_membership,
        collision_filter: anna_filter,
        collision_shape: ColliderShapeType::Capsule,
        x: 16.,
        y: 10.,
        sensor: false,
    };
    
    add_sensor_collision_bundle(&mut commands, bundle);

}

pub fn despawn_anna(mut commands: Commands, player_query: Query<Entity, With<Anna>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn handle_anna_death(
    mut death_event: EventReader<AnnaDeath>,
    mut game_over_event_writer: EventWriter<GameOver>,
    anna_stats: Res<AnnaStats> 
) {
    for AnnaDeath in death_event.iter() {
        println!("ANNNNNNNNA HAS DIEEEED");
        game_over_event_writer.send(GameOver { score: anna_stats.score as u32 });
    }
}

pub fn anna_received_damage(
    mut commands: Commands,
    mut anna: Query<(Entity, &Anna)>,
    mut anna_stats: ResMut<AnnaStats>,
    mut injured_event: EventReader<AnnaInjured>,
    mut death_writer: EventWriter<AnnaDeath>
) {
    for ouchy in injured_event.iter() {
        anna_stats.health -= ouchy.damage;
        println!("{} is Anna's health.",anna_stats.health);
    }
    if anna_stats.health <= 0.0 {
        if let Ok((anna_entity,_anna_gurr)) = anna.get_single_mut() {
            commands.entity(anna_entity).despawn_recursive();
            death_writer.send(AnnaDeath);
        }

    }
}

//This function dictates player movement for the controlable character, Anna.
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Anna>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

//This function defines x-min/max and y-min/max for player movement boundaries.
//We used the PrimaryWindow and scaled it down by a factor of 0.7 to match our level scene.
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Anna>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = -(window.width() * 0.7 / 2.0) + 16.0;
        let x_max = window.width() * 0.7 / 2.0 - 16.0;
        let y_min = -(window.height() * 0.7 /2.0) + 16.0;
        let y_max = window.height() * 0.7 /2.0 - 32.0;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;


/*ANNA PLUGIN */
pub struct AnnaPlugin;

impl Plugin for AnnaPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnnaStats::default())
            .add_event::<AnnaInjured>()
            .add_event::<AnnaDeath>()
            .add_event::<UpdateAnnaScore>()
            // Configure System Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_system(setup_anna.in_schedule(OnEnter(AppState::InGame)))
            // Systems
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    anna_received_damage, 
                    handle_anna_death,
                    update_score
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_anna.in_schedule(OnExit(AppState::InGame)));
    }
}
