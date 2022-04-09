use bevy::gltf::Gltf;
use bevy::prelude::*;
//use bevy_asset_loader::{AssetCollection, AssetLoader};
//use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_game_assets);
    }
}

fn init_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ModelAssets {
        player: asset_server.load("models/player.glb"),
        trunk: asset_server.load("models/trunk.glb"),
    });
}

pub struct ModelAssets {
    pub player: Handle<Gltf>,
    pub trunk: Handle<Gltf>,
}
