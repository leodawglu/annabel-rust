/***
 * This file contains objectives, goals, and interactable items that may
 * spawn within the game. This includes flowers that you can pick up for power-ups,
 * tokens to collect for points, and enemies that you may need to avoid.
 */

use bevy::prelude::*;
use bevy::window::{PrimaryWindow};
use rand::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::boss_fight::{FlowieBoss, SunFlower};
use crate::game::physics::colliders::*;

use super::SimulationState;
use crate::AppState;

pub const NUMBER_OF_FLOWERS: usize = 10;
pub const REAP_SUNFLOWER_TIMER: f32 = 2.0;
pub const GEN_SUNFLOWER_TIMER: f32 = 1.0;
pub const SUNFLOWER_COLLISION_SHAPE_RADIUS: f32 = 20.0;

#[derive(Component)]
pub struct Enemy();

#[derive(Component)]
pub struct SunFlowerTimer {
    pub sunflowertimer: Timer
}

#[derive(Component)]
pub struct SunFlowerReapTimer {
    pub sunflower_reap_timer: Timer
}

impl Default for SunFlowerReapTimer {
    fn default() -> Self {
        SunFlowerReapTimer {
            sunflower_reap_timer: Timer::from_seconds(REAP_SUNFLOWER_TIMER, TimerMode::Repeating)
        }
    }
}

pub struct SpawnSunflowerEvent;

impl Default for SunFlowerTimer {
    fn default() -> Self {
        SunFlowerTimer {
            sunflowertimer: Timer::from_seconds(GEN_SUNFLOWER_TIMER, TimerMode::Repeating)
        }
    }
}

//This function spawns sunflower power-ups that the player can pick up. These power-ups grant shield
//to protect against the thorn attacks.
//Uses rand::prelude crate to generate a x,y-coordinate within the bounds of the window view
pub fn spawn_flowers(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut _big_flowie_query: Query<Entity,With<FlowieBoss>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let spawn_width = (window.width() * 0.7) / 2.0 - 200.0;
    let spawn_height = (window.height() * 0.7) / 2.0 - 100.0;
    let mut rng = rand::thread_rng();

    for _i in 0..NUMBER_OF_FLOWERS {
        let random_x = rng.gen_range(-spawn_width..spawn_width);
        let random_y = rng.gen_range(-spawn_height..spawn_height);

        let sun_flower = commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("art/sunflower1.png"),
                ..default()
            },
            Enemy {},
            SunFlower {},
        )).id();

        let sunflower_membership: Group = Group::from_bits_truncate(0b0010);
        let sunflower_filter: Group = Group::from_bits_truncate(0b0001);

        let bundle = SensorCollisionBundle {
            entity: sun_flower,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            membership: sunflower_membership,
            collision_filter: sunflower_filter,
            collision_shape: ColliderShapeType::Ball,
            x: SUNFLOWER_COLLISION_SHAPE_RADIUS,
            y: SUNFLOWER_COLLISION_SHAPE_RADIUS,
            sensor: true,
        };

        add_sensor_collision_bundle(&mut commands, bundle);
    
        //generate sun_flower reap timer here

        let reap_timer = commands.spawn(
            SunFlowerReapTimer::default()
        ).id();

        commands.entity(sun_flower).add_child(reap_timer);
    }
    commands.spawn(SunFlowerTimer::default());

}





pub fn tick_sunflower_timer(
    time: Res<Time>,
    mut sunflowertimer: Query<&mut SunFlowerTimer>,
    mut spawn_sunflower_event: EventWriter<SpawnSunflowerEvent>
) {
    if let Ok(mut sun_ticker) = sunflowertimer.get_single_mut() {
        sun_ticker.sunflowertimer.tick(time.delta());
        if sun_ticker.sunflowertimer.just_finished() {
            spawn_sunflower_event.send(SpawnSunflowerEvent);
        }
    }
}

//Function very similar to function above, but it takes a timer event to spawn
//a flower after a certain number of seconds
pub fn spawn_single_flower(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut spawn_new_event: EventReader<SpawnSunflowerEvent>
) {
    for _lets_make_sun in spawn_new_event.iter() {
        let window = window_query.get_single().unwrap();
        let spawn_width = (window.width() * 0.7) / 2.0 - 200.0;
        let spawn_height = (window.height() * 0.7) / 2.0 - 100.0;
        let mut rng = rand::thread_rng();

        for _i in 0..1 {
            let random_x = rng.gen_range(-spawn_width..spawn_width);
            let random_y = rng.gen_range(-spawn_height..spawn_height);

            let new_sunflower = commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("art/sunflower1.png"),
                    ..default()
                },
                SunFlower {},
                Enemy {},
            )).id();
            let sunflower_membership: Group = Group::from_bits_truncate(0b0010);
            let sunflower_filter: Group = Group::from_bits_truncate(0b0001);

            let bundle = SensorCollisionBundle {
                entity: new_sunflower,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                membership: sunflower_membership,
                collision_filter: sunflower_filter,
                collision_shape: ColliderShapeType::Ball,
                x: SUNFLOWER_COLLISION_SHAPE_RADIUS,
                y: SUNFLOWER_COLLISION_SHAPE_RADIUS,
                sensor: true,
            };
    
            add_sensor_collision_bundle(&mut commands, bundle);

            let reap_timer = commands.spawn(
                SunFlowerReapTimer::default()
            ).id();

            commands.entity(new_sunflower).add_child(reap_timer);
        }

    }
}

//Tick the reap timer for the sunflowers
pub fn tick_sunflower_reap_timer(
    time: Res<Time>,
    mut parent_query: Query<(&mut SunFlower,Entity, &Children)>,
    mut child_query: Query<&mut SunFlowerReapTimer>,
    mut commands: Commands
) {

    for (_sunflower,sunflower_entity, children) in parent_query.iter_mut() {
        for reap_timer in children.iter() {
            if let Ok(mut reap_timer_comp) = child_query.get_mut(*reap_timer) {
                reap_timer_comp.sunflower_reap_timer.tick(time.delta());
                if reap_timer_comp.sunflower_reap_timer.just_finished() {
                    commands.entity(sunflower_entity).despawn_recursive();
                }
            }

        }
    }

}

pub struct ObjectivesPlugin;

impl Plugin for ObjectivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnSunflowerEvent>()
            // On Enter State
            .add_system(spawn_flowers.in_schedule(OnEnter(AppState::InGame)))
            // Systems
            .add_systems(
                (
                    spawn_single_flower,
                    tick_sunflower_timer,
                    tick_sunflower_reap_timer
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
