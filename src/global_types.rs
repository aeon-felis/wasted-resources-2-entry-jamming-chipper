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
pub struct PlayerControlled;
