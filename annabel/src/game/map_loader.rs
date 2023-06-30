/***
 * The purpose of this file is to serve as a plugin for tilemap loading functions
 * to be used anywhere where a sprite is needed such as the level map, the player
 * character, or other assets existing in the game. This plugin will be added to 
 * the system startup and will pre-load before spawn_camera & spawn_flowie occur.
 */

use bevy::prelude::*;
use crate::spawn_camera_1;
use crate::game::spawn_flowie_boss_scene;
use crate::TILE_SIZE;

#[derive(Resource, Default)]
pub struct MeadowsMapSheet(Handle<TextureAtlas>);
pub struct MapLoaderPlugin;

impl Plugin for MapLoaderPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(load_tilemap
            .before(spawn_flowie_boss_scene)
            .before(spawn_camera_1)
        );
    }
}

//Public "meadows" tilemap sprite creation function. Uses the atlas handle, index, and translation
//and spits out an Entity.
pub fn spawn_tilemap_sprite(
    commands: &mut Commands,
    tilemap: &MeadowsMapSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands.spawn(SpriteSheetBundle{
        sprite,
        texture_atlas: tilemap.0.clone(),
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    }).id()
}

//This function uses TextureAtlas to load a tilemap and store the asset as a resource for
//the system to preload on startup
pub fn load_tilemap(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
        let image = assets.load("art/meadows_map_tiles_v6.png");
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::new(30.0, 30.0),
            16,
            14,
            Some(Vec2::new(5.0, 5.0)),
            None
        );
        // let empty = TextureAtlas::is_empty(&atlas);
        let atlas_handle = texture_atlases.add(atlas);
        commands.insert_resource(MeadowsMapSheet(atlas_handle));
    }