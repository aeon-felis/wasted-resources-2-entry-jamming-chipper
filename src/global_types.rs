use bevy::prelude::*;
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
    pub jump_time_coefficient: f32,
    pub jump_potential: f32,
}

#[derive(Component)]
pub enum PlayerLeg {
    Right,
    Left,
}
