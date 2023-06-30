/***
 * This file uses the vanilla TextureAtlas functionalities to load a tilemap.
 * Our custom tilemap is first parsed and split up into individual tiles to be
 * loaded into atlas.
 * Then we have a custom function that iterates through a text file.
 * It reads from a textfile called MeadowsMap.txt and based on the characters
 * in each line, the value will be converted to usize to be used as an index
 * into the tilemap from atlas. The tile that was indexed would then be spawned
 * into camera view.
 */
use bevy::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use crate::game::map_loader::MeadowsMapSheet;
use crate::TILE_SIZE;
use crate::game::map_loader::spawn_tilemap_sprite;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MeadowsMapSheet>()
        .add_startup_system(load_meadows_map);
    }
}

/***
 * This function reads a textfile MeadowsMap.txt and reads each character in each line to be used
 * as a usize value to index our pre-loaded tilemap sprite in Texture Atlas. It will then call
 * spawn_tilemap_sprite to spawn the selected tile at the translation position.
 */
fn load_meadows_map(mut commands: Commands, tilemap: Res<MeadowsMapSheet>) {
    let file = File::open("assets/art/MeadowsMap.txt").expect("No existing map file found.");
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                spawn_tilemap_sprite(
                    &mut commands,
                    &tilemap,
                    char as usize,
                    Color::rgb(0.9,0.9,0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0)
                );
            }   
        }
    }
}