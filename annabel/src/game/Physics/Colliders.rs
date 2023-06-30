use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use crate::game::boss_fight::*;
use crate::game::player::{AnnaInjured, UpdateAnnaScore};
use crate::game::Thorn;

//This function is directly influenced from Casey Bailey's GitLab Rust project mentioned in
//the readme

#[derive(PartialEq, Eq)]
pub enum ColliderShapeType {
    Ball,
    Cuboid,
    Capsule,
}

pub struct CustomCollisionEvent {
    entity1: Entity,
    entity2: Entity
}
//Bundled all the neccesary components for a collision sensor
pub struct SensorCollisionBundle {
    pub entity: Entity,
    pub transform: Transform,
    pub membership: Group,
    pub collision_filter: Group,
    pub collision_shape: ColliderShapeType,
    pub x: f32,
    pub y: f32,
    pub sensor: bool,
}

//Function should not be directly called by game loop
//It should be called by other entities to create a collision sensor
pub fn add_sensor_collision_bundle(
    commands: &mut Commands,
    bundle: SensorCollisionBundle
) {
    let new_collision_shape: Entity = match bundle.collision_shape {
        ColliderShapeType::Ball => {
            commands
                .spawn(Collider::ball(bundle.x))
                .insert(CollisionGroups::new(bundle.membership,bundle.collision_filter))
                .insert(ActiveCollisionTypes::all())
                .insert(ActiveEvents::all())
                .insert(TransformBundle::from_transform(bundle.transform))
                .id()
        }
        ColliderShapeType::Cuboid => {
            commands
                .spawn(Collider::cuboid(bundle.x,bundle.y))
                .insert(CollisionGroups::new(bundle.membership,bundle.collision_filter))
                .insert(ActiveCollisionTypes::all())
                .insert(ActiveEvents::all())
                .insert(TransformBundle::from_transform(bundle.transform))
                .id()
        }
        ColliderShapeType::Capsule => {
            commands
                .spawn(Collider::capsule_y(bundle.y,bundle.x))
                .insert(CollisionGroups::new(bundle.membership,bundle.collision_filter))
                .insert(ActiveEvents::all())
                .insert(ActiveCollisionTypes::all())
                .insert(TransformBundle::from_transform(bundle.transform))
                .id()
        }
    };

    if bundle.sensor {
        commands.entity(new_collision_shape).insert(Sensor);
    }

    commands.entity(bundle.entity).add_child(new_collision_shape);
}

/* This was taken directly from bevy_rapier2d official documentation examples */
/* Then combined with Casey Baileys code and then we added some logic for our use case */
pub fn display_events(
    mut _commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut _contact_force_events: EventReader<ContactForceEvent>,
    mut collision_event_propogation: EventWriter<CustomCollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        //println!("Received collision event: {:?}", collision_event);
        if let CollisionEvent::Started(ent1,ent2,_flags) = collision_event {
            //handle this event with different systems that are dependent on parent-child queries
            collision_event_propogation.send(
                CustomCollisionEvent {
                    entity1: *ent1,
                    entity2: *ent2,
                }
            );
        };
    }

}

//Function makes a query that involves Anna, if
//it involves Anna, Anna's stats will be adjusted
pub fn process_damage_event(
    mut collision_event: EventReader<CustomCollisionEvent>,
    mut anna_injured: EventWriter<AnnaInjured>,
    mut parent_query: Query<&mut Thorn>,
    mut child_query: Query<&Parent, With<Collider>>,
    mut commands: Commands
) {
    for this_event in collision_event.iter() {
        if let Ok(parent) = child_query.get_mut(this_event.entity1) {
            if let Ok(_thorn) = parent_query
                .get_mut(parent.get()) {
                anna_injured.send(AnnaInjured {
                    damage: 1.0
                });
                commands.entity(parent.get()).despawn_recursive();
                continue;
            }
        }
        if let Ok(parent) = child_query.get_mut(this_event.entity2) {
            if let Ok(_thorn) = parent_query
                .get_mut(parent.get()) {
                anna_injured.send(AnnaInjured {
                    damage: 1.0
                });
                commands.entity(parent.get()).despawn_recursive();
                continue;
            }
        }
    }
}

//Function makes a query that involves Anna, if
//it involves Anna, Anna's stats will be adjusted
pub fn process_collect_sunflower_event(
    mut collision_event: EventReader<CustomCollisionEvent>,
    mut parent_query: Query<(&mut SunFlower, Entity)>,
    mut child_query: Query<&Parent, With<Collider>>,
    mut commands: Commands,
    mut score_event: EventWriter<UpdateAnnaScore>
) {
    for this_event in collision_event.iter() {
        if let Ok(parent) = child_query.get_mut(this_event.entity1) {
            if let Ok((_sunflower,_sunflower_entity)) = parent_query
                .get_mut(parent.get()) {
                commands.entity(parent.get()).despawn_recursive();
                score_event.send(UpdateAnnaScore);
                continue;
            }
        }
        if let Ok(parent) = child_query.get_mut(this_event.entity2) {
            if let Ok((_sunflower,_sunflower_entity)) = parent_query
                .get_mut(parent.get()) {
                commands.entity(parent.get()).despawn_recursive();
                score_event.send(UpdateAnnaScore);
                continue;
            }
        }
    }
}
