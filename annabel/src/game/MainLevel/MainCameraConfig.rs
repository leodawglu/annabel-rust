use bevy::prelude::*;

#[derive(Component)]
pub struct MainLevelCamera;

//Camera is child of Anna so it follows her movements
//spawned in MainLevel scene not as Game System
pub fn spawn_main_level_camera(
    mut commands: &mut Commands,
) ->Entity {
    commands.spawn((Camera2dBundle {
        projection: {
            OrthographicProjection {
                scale: 0.5,
                ..default()
            }
        },
        ..default()
    },
                    MainLevelCamera
    )).id()
}