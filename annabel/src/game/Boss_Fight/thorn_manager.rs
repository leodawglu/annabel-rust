//This file manages Flowie boss thorn attacks
//It will be responsible for generating thorns
//in different patterns

//So we need some systems to generate thorns at a certain location,
//with a direction, and a speed

//Could make resource for flowie maw position
//make child of flowie?

//first start with the degenerate case:
//generate one thorn going straight down
// use std::ptr::null;
// use core::option::Option;

use bevy::prelude::*;
use std::f32::consts::PI;

use crate::game::physics::colliders::*;
use bevy::window::PrimaryWindow;
use crate::game::boss_fight::FlowieBoss;
//already declared in module tree so doesn't need a mod declaration
use crate::game::boss_fight::thorns::*;
use bevy_rapier2d::prelude::*;
use crate::game::SimulationState;
use crate::AppState;

pub const NUM_THORNS_IN_FAN: u16 = 32;

pub struct ShootThornFanEvent {}
pub struct SwitchAttackEvent {}
pub struct UpdateAttackEvent {
    pub new_mode: AttackModeList
}

#[derive(Component)]
pub struct ThornManager {}

#[derive(Component)]
pub struct AttackMode {
    pub attack_mode: AttackModeList
}

impl Default for AttackMode {
    fn default() -> Self {
        AttackMode {
            attack_mode: AttackModeList::ThornFan
        }
    }
}

#[derive(PartialEq)]
pub enum AttackModeList {
    ThornFan,
    ThornThicket
}

#[derive(Component)]
pub struct ThornFanShootAngles {
    list_of_directions: [f32;3],
    current_direction_index: usize,
}

impl Default for ThornFanShootAngles {
    fn default() -> Self {
        ThornFanShootAngles {
            list_of_directions: [-45.0,-90.0,-135.0],
            current_direction_index: 0,
        }
    }
}

impl ThornFanShootAngles {
    //Make sure index loops back to zero when it exceeds the length
    fn increase_index(&mut self) {
        self.current_direction_index =
            (self.current_direction_index + 1) %
                (self.list_of_directions.len())
    }
}


pub const THORN_FAN_COOLDOWN: f32 = 3.0;
pub const SWITCH_THORN_ATTACKS: f32 = 10.0;


#[derive(Component)]
pub struct SwitchThornAttackTimer {
    pub timer: Timer,
}

impl Default for SwitchThornAttackTimer {
    fn default() -> Self {
        SwitchThornAttackTimer {
            timer: Timer::from_seconds(SWITCH_THORN_ATTACKS, TimerMode::Repeating)
        }
    }
}

#[derive(Component)]
pub struct ThornFanTimer {
    pub timer: Timer,
}

impl Default for ThornFanTimer {
    fn default() -> Self {
        ThornFanTimer {
            timer: Timer::from_seconds(THORN_FAN_COOLDOWN, TimerMode::Repeating)
        }
    }
}

impl ThornFanTimer {
    fn increment_timer(
        &mut self, game_loop_timer: &Res<Time>,
        mut _commands: Commands,
        _window: &Window,
        mut shoot_event: EventWriter<ShootThornFanEvent>,
        _asset_server: &Res<AssetServer>
    ) {

        self.timer.tick(game_loop_timer.delta());

        if self.timer.just_finished() {
            shoot_event.send(ShootThornFanEvent {});

            self.timer.reset();
        }
        
    }
}

//Handles the switching of attacks using attack signals

//Only tick the timer if flowie boss is spawned, maybe redundant given it will only be
//spawned when FlowieBoss is spawned, but good for safety
pub fn tick_thorn_fan_timer(
    commands: Commands,
    mut thorn_fan_timer_query: Query<&mut ThornFanTimer, With<FlowieBoss>>,
    shoot_event: EventWriter<ShootThornFanEvent>,
    mut update_attack_mode: EventReader<UpdateAttackEvent>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window: &Window = window_query.get_single().unwrap();
    for update_mode in update_attack_mode.iter() {
        if update_mode.new_mode == AttackModeList::ThornThicket {

        }
    }
    if let Some(mut thorn_fan_time) = thorn_fan_timer_query.iter_mut().next() {
        thorn_fan_time.increment_timer(&time, commands, window, shoot_event, &asset_server);
    }
}

//function takes shootThornFan Event reader
//FlowieBoss entity(for position)
//asset_server to render thorns
//translation to change position

//Function will shoot thorn cluster  -45,-90, or -135 degrees with respect to positive x-axis
//This attack is avoidable
pub fn shoot_thorn_fan(
    mut fan_shoot_signal: EventReader<ShootThornFanEvent>,
    mut fan_angles_query: Query<&mut ThornFanShootAngles>,
    flowie_boss_query: Query<&Transform, With<FlowieBoss>>,
    asset_server: Res<AssetServer>,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands
) {
    if let Ok(mut shoot_angles) = fan_angles_query.get_single_mut() {
        let current_angle_index = shoot_angles.current_direction_index;
        let current_angle = shoot_angles.list_of_directions[current_angle_index];
        let flowie_transform = flowie_boss_query.get_single().unwrap();
        let start_angle: f32 = current_angle - f32::from(NUM_THORNS_IN_FAN/2);
        for _shoot in fan_shoot_signal.iter() {
            for shifted_pi_halves in 0..4 {
                let shifted: f32 = (shifted_pi_halves as f32) * (PI/2.0);
                for i in 0..NUM_THORNS_IN_FAN {
                    let angle_to_shoot = f32::to_radians(start_angle + f32::from(i))-shifted;
                    let direction_vector = Vec2::from_angle(angle_to_shoot);
                    let thorn: Thorn = Thorn::new(10.0,direction_vector.x,direction_vector.y,200.0);
                    let thorn_instance = commands.spawn((
                        thorn,
                        SpriteBundle {
                            transform: *flowie_transform,
                            texture: asset_server.load("art/Thorn_Placeholder(1).png"),
                            ..default()
                        }
                    )).id();

                    commands.entity(thorn_instance).insert(RigidBody::KinematicPositionBased);

                    //add group info here
                    let thorn_membership: Group = Group::from_bits_truncate(0b0100);
                    //don't want throw sensing signal if big Flowie interacted with
                    let thorn_filter: Group = Group::from_bits_truncate(0b0001);

                    let bundle = SensorCollisionBundle {
                        entity: thorn_instance,
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        membership: thorn_membership,
                        collision_filter: thorn_filter,
                        collision_shape: ColliderShapeType::Cuboid,
                        x: 20.0,
                        y: 20.0,
                        sensor: true,
                    };
                    add_sensor_collision_bundle(&mut commands, bundle);

                }
            }

            shoot_angles.increase_index();
        }
    }
}

pub struct ThornMechanicsPlugin;

impl Plugin for ThornMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootThornFanEvent>()
            .add_event::<SwitchAttackEvent>()
            .add_event::<UpdateAttackEvent>()
            .add_event::<CustomCollisionEvent>()
            .add_systems(
                (
                    shoot_thorn,
                    shoot_thorn_fan,
                    process_damage_event,
                    process_collect_sunflower_event,
                    tick_thorn_fan_timer,
                    tick_thorn_reap_timer,
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(
                (
                    display_events.after(shoot_thorn_fan).after(shoot_thorn)
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running))
            );
    }
}