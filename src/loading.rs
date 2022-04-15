use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
//use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ModelAssets>();
        app.init_collection::<FontAssets>();
    }
}

#[derive(AssetCollection)]
pub struct ModelAssets {
    #[asset(path = "models/player.glb")]
    pub player: Handle<Gltf>,
    #[asset(path = "models/trunk.glb")]
    pub trunk: Handle<Gltf>,
    #[asset(path = "models/chipper.glb")]
    pub chipper: Handle<Gltf>,
    #[asset(path = "models/woodchip.glb")]
    pub woodchip: Handle<Gltf>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}
