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

 use bevy::app::AppExit;
 use bevy::prelude::*;

 use crate::game::game_ui::game_over_menu::components::*;
 use crate::game::game_ui::game_over_menu::styles::*;

 use crate::events::GameOver;

 
 pub fn interact_with_quit_button(
     mut app_exit_event_writer: EventWriter<AppExit>,
     mut button_query: Query<
         (&Interaction, &mut BackgroundColor),
         (Changed<Interaction>, With<QuitButton>),
     >,
 ) {
     for (interaction, mut color) in button_query.iter_mut() {
         match *interaction {
             Interaction::Clicked => {
                 *color = PRESSED_BUTTON.into();
                 app_exit_event_writer.send(AppExit);
             }
             Interaction::Hovered => {
                 *color = HOVERED_BUTTON.into();
             }
             Interaction::None => {
                 *color = NORMAL_BUTTON.into();
             }
         }
     }
 }

 pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_MENU_STYLE,
                z_index: ZIndex::Local(2), // See Ref. 1
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: GAME_OVER_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Final Score Text
                    
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    // Score not printing, needs work done
                                    "Your final score: ",
                                    get_final_score_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText {},
                    ));
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit Game",
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOver>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_event_reader.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score.to_string());
        }
    }
}
