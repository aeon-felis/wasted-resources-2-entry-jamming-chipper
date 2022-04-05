mod arena;
mod camera;
mod input;
mod player;

use bevy::ecs::schedule::ShouldRun;
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
        app.add_plugin(input::InputPlugin);
        app.add_plugin(arena::ArenaPlugin);
        app.add_plugin(player::PlayerPlugin);
        app.add_system_set({
            SystemSet::on_enter(AppState::ClearLevelAndThenLoad)
                .with_system(clear_and_load)
                .with_system(create_move_to_state_system(AppState::LoadLevel))
        });
        app.add_system_set({
            SystemSet::on_enter(AppState::LoadLevel)
                .with_system(create_move_to_state_system(AppState::Game))
        });
        app.add_system(enable_disable_physics.with_run_criteria(run_on_state_change));
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

pub fn run_on_state_change(
    state: Res<State<AppState>>,
    mut prev_state: Local<Option<AppState>>,
) -> ShouldRun {
    let state = state.current();
    if Some(state) == (&*prev_state).as_ref() {
        return ShouldRun::No;
    }
    *prev_state = Some(state.clone());
    ShouldRun::Yes
}

fn enable_disable_physics(
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
) {
    let set_to = match state.current() {
        AppState::Game => true,
        AppState::Menu(_) | AppState::ClearLevelAndThenLoad | AppState::LoadLevel => false,
    };
    rapier_configuration.physics_pipeline_active = set_to;
    rapier_configuration.query_pipeline_active = set_to;
}
