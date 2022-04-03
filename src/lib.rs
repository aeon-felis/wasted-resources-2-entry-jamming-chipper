mod game_systems;
pub mod global_types;
mod loading;
mod menu;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use self::game_systems::GameSystemsPlugin;
use self::global_types::{AppState, MenuState};
use self::loading::LoadingPlugin;
use self::menu::MenuPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu(MenuState::Main));
        app.add_plugin(LoadingPlugin);
        app.add_plugin(MenuPlugin);
        app.add_plugin(GameSystemsPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
