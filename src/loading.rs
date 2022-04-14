use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_hanabi::{EffectAsset, Spawner, ColorOverLifetimeModifier, Gradient, PositionSphereModifier, ShapeDimension, AccelModifier, SizeOverLifetimeModifier};
//use bevy_asset_loader::{AssetCollection, AssetLoader};
//use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_game_assets);
    }
}

fn init_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut particle_effects_assets: ResMut<Assets<EffectAsset>>,
) {
    commands.insert_resource(ModelAssets {
        player: asset_server.load("models/player.glb"),
        trunk: asset_server.load("models/trunk.glb"),
        chipper: asset_server.load("models/chipper.glb"),
        woodchip: asset_server.load("models/woodchip.glb"),
    });

    commands.insert_resource(ParticleEffectsAssets {
        chipping_wood: particle_effects_assets.add({
            EffectAsset {
                name: "ChippingWood".to_string(),
                capacity: 1000,
                spawner: Spawner::rate(5.0.into()),
                ..Default::default()
            }
            .init(PositionSphereModifier {
                center: Vec3::ZERO,
                radius: 2.0,
                dimension: ShapeDimension::Surface,
                speed: 6.0.into(),
            })
            // Every frame, add a gravity-like acceleration downward
            .update(AccelModifier {
                accel: Vec3::new(0., -3., 0.),
            })
            .render(ColorOverLifetimeModifier {
                gradient: {
                    let mut gradient = Gradient::new();
                    gradient.add_key(0.0, Vec4::new(0.44, 0.33, 0.23, 1.0));
                    gradient.add_key(1.0, Vec4::new(0.44, 0.33, 0.23, 0.0));
                    // gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
                    // gradient.add_key(1.0, Vec4::splat(0.));
                    gradient
                },
            })
            .render(SizeOverLifetimeModifier {
                gradient: {
                    let mut gradient = Gradient::new();
                    gradient.add_key(0.0, Vec2::splat(0.5));
                    gradient.add_key(1.0, Vec2::splat(0.0));
                    gradient
                }
            })
        }),
    });
}

pub struct ModelAssets {
    pub player: Handle<Gltf>,
    pub trunk: Handle<Gltf>,
    pub chipper: Handle<Gltf>,
    pub woodchip: Handle<Gltf>,
}

pub struct ParticleEffectsAssets {
    pub chipping_wood: Handle<EffectAsset>,
}
