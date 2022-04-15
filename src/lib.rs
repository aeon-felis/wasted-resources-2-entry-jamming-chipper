mod game_systems;
pub mod global_types;
pub mod gltf_spawner;
mod loading;
mod menu;
mod score_display;
mod utils;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use self::game_systems::GameSystemsPlugin;
use self::global_types::{AppState, MenuState};
use self::gltf_spawner::GltfSpawnerPlugin;
use self::loading::LoadingPlugin;
use self::menu::MenuPlugin;
use self::score_display::ScoreDisplayPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu(MenuState::Main));
        app.add_plugin(LoadingPlugin);
        app.add_plugin(GltfSpawnerPlugin);
        app.add_plugin(MenuPlugin);
        app.add_plugin(GameSystemsPlugin);

        app.add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(UiCameraBundle::default());
        });
        app.add_plugin(ScoreDisplayPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
