use bevy::prelude::*;
use crate::player::*;
use bevy_rapier2d::prelude::*;
use crate::Physics::Colliders::*;
use MainCameraConfig::*;

pub mod MainCameraConfig;
pub mod setup_main_level;

#[derive(Component)]
pub struct MainLevel {}

//Spawn Anna
//Put Camera as child of Anna
pub fn setup_main_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let anna_membership: Group = Group::from_bits_truncate(0b001);
    let anna_filter: Group = Group::from_bits_truncate(0b0100);

    let anna_entity = setup_anna(&mut commands, &asset_server);
    let bundle = SensorCollisionBundle {
        entity: anna_entity,
        transform: Transform::from_xyz(0., 0., 0.),
        membership: anna_membership,
        collision_filter: anna_filter,
        collision_shape: ColliderShapeType::CAPSULE,
        x: 16.,
        y: 10.,
        sensor: false,
    };
    
    add_sensor_collision_bundle(&mut commands, bundle);

    let main_camera: Entity = spawn_main_level_camera(&mut commands);

    commands.entity(anna_entity).add_child(main_camera);
}