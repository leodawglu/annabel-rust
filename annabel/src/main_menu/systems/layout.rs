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

use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use crate::systems::MainMenuCamera;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>, camera_query: Query<Entity, With<MainMenuCamera>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Game Title (Text)
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Annabel's\nAdventure",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Start Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    StartButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Start Game",
                                get_button_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Exit Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    ExitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Exit",
                                get_button_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    main_menu_entity
}
