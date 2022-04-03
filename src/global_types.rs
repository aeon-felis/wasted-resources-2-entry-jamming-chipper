use bevy::prelude::*;

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
