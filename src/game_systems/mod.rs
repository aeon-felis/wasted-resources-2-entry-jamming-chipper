mod arena;
mod camera;

use bevy::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel};

pub struct GameSystemsPlugin;

fn create_move_to_state_system(new_state: AppState) -> impl Fn(ResMut<State<AppState>>) {
    move |mut state: ResMut<State<AppState>>| {
        state.set(new_state.clone()).unwrap();
    }
}

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(camera::CameraPlugin);
        app.add_plugin(arena::ArenaPlugin);
        app.add_system_set({
            SystemSet::on_enter(AppState::ClearLevelAndThenLoad)
                .with_system(clear_and_load)
                .with_system(create_move_to_state_system(AppState::LoadLevel))
        });
        app.add_system_set({
            SystemSet::on_enter(AppState::LoadLevel)
                .with_system(create_move_to_state_system(AppState::Game))
        });
    }
}

fn clear_and_load(
    mut commands: Commands,
    entities_to_despawn: Query<Entity, With<DespawnWithLevel>>,
) {
    for entity in entities_to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
