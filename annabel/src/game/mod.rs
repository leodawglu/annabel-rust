pub mod boss_fight;
mod tilemap;
mod map_loader;
mod physics;
mod player;
mod game_ui;

mod systems;
mod objectives;

pub use boss_fight::{spawn_flowie_boss_scene, thorn_manager::*, thorns::*};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::window::ExitCondition::OnPrimaryClosed;
use bevy::window::WindowResolution;
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::prelude::*;
use player::AnnaPlugin;
use crate::game::game_ui::GameUIPlugin;
use crate::game::boss_fight::BossFightPlugin;

//added for game UI
use crate::events::GameOver;
use systems::*;
use objectives::*;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::InGame)))
            // My Plugins
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: produce_window(),
                    exit_condition: OnPrimaryClosed,
                    close_when_requested: false,
            }).set(ImagePlugin::default_nearest()))

            // Plugins
            .add_plugin(EditorPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())

            // Resources
            .insert_resource(LevelSelection::Index(0))

            //Custom Plugins

            .add_plugin(AnnaPlugin)
            .add_plugin(ThornMechanicsPlugin)
            .add_plugin(ObjectivesPlugin)
            .add_plugin(BossFightPlugin)
            .add_plugin(GameUIPlugin)

            .add_system(toggle_simulation.run_if(in_state(AppState::InGame)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::InGame)));
    }
}

//This defines the window size of the game view
fn produce_window() -> Option<Window> {
    let game_window = Window {
        resolution: WindowResolution::new(1600.0,1000.0),
        ..default()
    };
    Some(game_window)
}

/***
 * This struct is what LDTK uses as a sprite sheet bundle.
 */
#[derive(Bundle, LdtkEntity)]
pub struct MeadowsMapBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
