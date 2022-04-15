mod arena;
mod camera;
mod chippers;
mod input;
mod player;
mod trunks;
mod woodchips;

use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_hanabi::ParticleEffect;

use crate::global_types::{AppState, DespawnWithLevel};

pub struct GameSystemsPlugin;

struct MoveToStateIn {
    in_frames: usize,
    target_state: Option<AppState>,
}

fn apply_move_to_state_in(
    mut move_to_state_in: ResMut<MoveToStateIn>,
    mut state: ResMut<State<AppState>>,
) {
    if move_to_state_in.in_frames == 0 {
        return;
    }
    move_to_state_in.in_frames -= 1;
    if move_to_state_in.in_frames == 0 {
        if let Some(new_state) = move_to_state_in.target_state.take() {
            state.set(new_state).unwrap();
        }
    }
}

fn create_move_to_state_system(new_state: AppState) -> impl Fn(ResMut<MoveToStateIn>) {
    move |mut move_to_state_in| {
        *move_to_state_in = MoveToStateIn {
            in_frames: 1,
            target_state: Some(new_state.clone()),
        }
    }
}

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveToStateIn {
            in_frames: 0,
            target_state: None,
        });
        app.add_system(apply_move_to_state_in);
        app.add_plugin(camera::CameraPlugin);
        app.add_plugin(input::InputPlugin);
        app.add_plugin(arena::ArenaPlugin);
        app.add_plugin(player::PlayerPlugin);
        app.add_plugin(trunks::TrunksPlugin);
        app.add_plugin(chippers::ChippersPlugin);
        app.add_plugin(woodchips::WoodshipsPlugin);
        app.add_system_set({
            SystemSet::on_enter(AppState::ClearParticleEffects)
                .with_system(clear_particle_effects)
                .with_system(create_move_to_state_system(AppState::ClearLevelAndThenLoad))
        });
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

fn clear_particle_effects(mut particle_effects_query: Query<&mut ParticleEffect>) {
    for mut particle_effect in particle_effects_query.iter_mut() {
        if let Some(spawner) = particle_effect.maybe_spawner() {
            spawner.set_active(false);
        }
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
        AppState::Menu(_)
        | AppState::ClearParticleEffects
        | AppState::ClearLevelAndThenLoad
        | AppState::LoadLevel => false,
    };
    rapier_configuration.physics_pipeline_active = set_to;
    rapier_configuration.query_pipeline_active = set_to;
}
