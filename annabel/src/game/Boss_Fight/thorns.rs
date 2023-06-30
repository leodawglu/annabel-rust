// use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::time::TimerMode;


//Note::Instead of putting attributes of thorn in the thorn struct
//I think that adding them to plugin makes more sense.

//Note: To add collision object to thorn,
//use rapier to add Rigid Body and Collision
//shape. This will be a startup system
//See this page: https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies

pub const REAP_DURATION: f32 = 10.0;
pub const THORN_SPEED: f32 = 100.0;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Thorn {
    pub reap_timer: Timer,
    pub direction: Vec3,
    pub speed: f32
}

impl Default for Thorn {
    fn default() -> Self {
        Thorn {
            reap_timer: Timer::from_seconds(REAP_DURATION, TimerMode::Once),
            direction: Vec3::new(1.,0., 0.),
            speed: THORN_SPEED
        }
    }
}

impl Thorn {
    fn increment_timer(&mut self, game_loop_timer: &Res<Time>) {
        self.reap_timer.tick(game_loop_timer.delta());
    }
    pub fn new(reap_time: f32, dir_x: f32, dir_y: f32, speed: f32) -> Self {
        Self {
            reap_timer: Timer::from_seconds(reap_time, TimerMode::Once),
            direction: Vec3::new(dir_x,dir_y, 0.0),
            speed
        }
    }
}


//Function increments the reap timer, using Bevy's builtin Time resource
//When the timer ends, the thorn is reaped

//Should refactor to be based on event

pub fn tick_thorn_reap_timer(
    mut commands: Commands,
    mut thorn_query: Query<(&mut Thorn, Entity),With<Thorn>>,
    time: Res<Time>
) {
    for (mut thorn,thorn_entity) in thorn_query.iter_mut() {
        thorn.increment_timer(&time);
        if thorn.reap_timer.just_finished() {
            commands.entity(thorn_entity).despawn();
        }
    }
}

pub fn shoot_thorn(
    mut thorn_query: Query<(&mut Transform, &Thorn), With<Thorn>>,
    time: Res<Time>
) {
    for (mut transform, thorn) in thorn_query.iter_mut() {
        transform.translation += thorn.direction * thorn.speed * time.delta_seconds();
    }
}


