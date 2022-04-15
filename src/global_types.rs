use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier2d::na::Vector2;
use ezinput::prelude::BindingTypeView;
use ezinput_macros::BindingTypeView;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu(MenuState),
    ClearLevelAndThenLoad,
    LoadLevel,
    Game,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum MenuState {
    Main,
    Pause,
}

#[derive(Component)]
pub struct DespawnWithLevel;

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputBinding {
    MoveHorizontal,
    Jump,
    Pause,
}

#[derive(Component)]
pub struct PlayerControl {
    pub max_speed: f32,
    pub impulse_coefficient: f32,
    pub jump_power_coefficient: f32,
    pub jump_from_woodchip_power_coefficient: f32,
    pub jump_time_coefficient: f32,
    pub jump_potential: f32,
    pub last_stood_on: Vector2<f32>,
    pub stood_on_potential: f32,
    pub stood_on_time_coefficient: f32,
    pub uphill_move_efficiency: f32,
    pub uphill_stop_efficiency: f32,
}

#[derive(Component)]
pub enum Chipper {
    Free,
    Jammed,
}

#[derive(Component)]
pub enum Trunk {
    Free,
    InChipper(HashSet<Entity>),
}

#[derive(Component)]
pub struct SpawnsWoodchips(pub Timer);

#[derive(Component)]
pub enum Woodchip {
    Free,
    StuckInChipper(Entity),
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum ParticleEffectType {
    ChippingWood,
    Smoke,
    Blood,
}

#[derive(Default)]
pub struct ScoreStatus {
    pub time: Duration,
    pub logs_chipped: u32,
    pub woodchips_cleared: u32,
}

impl ScoreStatus {
    pub fn format_time(&self) -> String {
        let time_in_seconds = self.time.as_secs_f32();
        let only_minutes = time_in_seconds as u32 / 60;
        let only_seconds = time_in_seconds % 60.0;
        format!("{:02}:{:04.1}", only_minutes, only_seconds)
    }
}
