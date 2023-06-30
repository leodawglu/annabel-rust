mod stats;
mod game_over_menu;

use bevy::prelude::*;

use stats::AnnaStatsPlugin;
use game_over_menu::GameOverMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugin(AnnaStatsPlugin)
            .add_plugin(GameOverMenuPlugin);
    }
}
