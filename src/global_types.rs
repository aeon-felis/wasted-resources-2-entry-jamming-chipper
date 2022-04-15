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
