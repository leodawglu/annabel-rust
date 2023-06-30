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
use crate::game::player::*;
use crate::game::game_ui::stats::components::*;
use crate::game::game_ui::stats::styles::*;
 
 pub fn spawn_display_annastats(mut commands: Commands, asset_server: Res<AssetServer>) {
     build_hud(&mut commands, &asset_server);
 }
 
 pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
     let hud_entity = commands
         .spawn((
             NodeBundle {
                 style: HUD_STYLE,
                 ..default()
             },
             Hud {},
         ))
         .with_children(|parent| {
             // Top Left 
             parent
                 .spawn(NodeBundle {
                     style: LHS_STYLE,
                     background_color: BACKGROUND_COLOR.into(),
                     ..default()
                 })
                 .with_children(|parent| {
                     // Sunflower Image
                     parent.spawn(ImageBundle {
                         style: IMAGE_STYLE,
                         image: asset_server.load("art/sunflower1.png").into(),
                         ..default()
                     });
                     // Score Text
                     parent.spawn((
                         TextBundle {
                             style: Style { ..default() },
                             text: Text {
                                 sections: vec![TextSection::new(
                                     "0",
                                     get_text_style(asset_server),
                                 )],
                                 alignment: TextAlignment::Center,
                                 ..default()
                             },
                             ..default()
                         },
                         ScoreText {},
                     ));
                 });
             // Top Right
             parent
                 .spawn(NodeBundle {
                     style: RHS_STYLE,
                     background_color: BACKGROUND_COLOR.into(),
                     ..default()
                 })
                 .with_children(|parent| {
                     // Health Text
                     parent.spawn((
                         TextBundle {
                             style: Style { ..default() },
                             text: Text {
                                 sections: vec![TextSection::new(
                                     "0",
                                     get_text_style(asset_server),
                                 )],
                                 alignment: TextAlignment::Center,
                                 ..default()
                             },
                             ..default()
                         },
                         HealthText {},
                     ));
                     // Heart Image
                     parent.spawn(ImageBundle {
                         style: IMAGE_STYLE,
                         image: asset_server.load("art/Hearts/PNG/basic/heart.png").into(),
                         ..default()
                     });
                 });
         })
         .id();
 
     hud_entity
 }
 
 pub fn despawn_display_annastats(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
     for entity in hud_query.iter() {
         commands.entity(entity).despawn_recursive();
     }
 }
 

 pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, anna_stats: Res<AnnaStats>) {
    if anna_stats.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", anna_stats.score.to_string());
        }
    }
}

pub fn update_health_text(mut text_query: Query<&mut Text, With<HealthText>>, anna_stats: Res<AnnaStats>) {
    if anna_stats.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", anna_stats.health.to_string());
        }
    }
}