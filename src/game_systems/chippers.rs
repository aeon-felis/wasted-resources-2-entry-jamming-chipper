use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_hanabi::ParticleEffect;
use bevy_rapier2d::prelude::*;
use bevy_tweening::lens::TransformRotateXLens;
use bevy_tweening::{Animator, EaseMethod, Tween, TweeningType};

use crate::global_types::{AppState, Chipper, DespawnWithLevel, Trunk};
use crate::gltf_spawner::{SpawnCollider, SpawnGltfNode};
use crate::loading::{ModelAssets, ParticleEffectsAssets};

pub struct ChippersPlugin;

impl Plugin for ChippersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_chippers));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(set_chipper_effect));
    }
}

fn setup_chippers(mut commands: Commands, model_assets: Res<ModelAssets>) {
    for x in -3..=3 {
        let mut cmd = commands.spawn();
        cmd.insert(Transform::identity());
        cmd.insert(GlobalTransform::identity());
        cmd.insert(Visibility::default());
        cmd.insert(ComputedVisibility::default());
        cmd.insert(GlobalTransform::identity());
        cmd.with_children(|commands| {
            for (z, lens) in [
                (
                    -0.5,
                    TransformRotateXLens {
                        start: 0.0,
                        end: 2.0 * std::f32::consts::PI,
                    },
                ),
                (
                    0.5,
                    TransformRotateXLens {
                        start: 2.0 * std::f32::consts::PI,
                        end: 0.0,
                    },
                ),
            ] {
                commands
                    .spawn()
                    .insert(GlobalTransform::identity())
                    .insert(Transform::from_xyz(0.0, 0.0, z))
                    .insert(SpawnGltfNode(model_assets.chipper.clone(), "Chipper"))
                    .insert(Animator::new(Tween::new(
                        EaseMethod::Linear,
                        TweeningType::Loop,
                        Duration::from_millis(1000),
                        lens,
                    )));
            }
        });
        cmd.insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: point![x as f32 * 2.1, -1.0].into(),
            ..Default::default()
        });
        cmd.insert(RigidBodyPositionSync::Discrete);
        cmd.insert(SpawnCollider {
            gltf: model_assets.chipper.clone(),
            node_name: "Collider",
            collider_type: ColliderType::Sensor,
            material: Default::default(),
            flags: ColliderFlags {
                active_events: ActiveEvents::INTERSECTION_EVENTS,
                ..Default::default()
            },
        });
        cmd.insert(Chipper::Free);
        cmd.insert(ChipperEffect::NoEffect);
        cmd.insert(DespawnWithLevel);
    }
}

#[derive(Component, PartialEq)]
enum ChipperEffect {
    NoEffect,
    ChippingWood,
    Smoking,
}

fn set_chipper_effect(
    mut commands: Commands,
    particle_effects_assets: Res<ParticleEffectsAssets>,
    trunks_query: Query<&Trunk>,
    mut chippers_query: Query<(Entity, &Chipper, &mut ChipperEffect)>,
) {
    let chipping_chippers: HashSet<Entity> = trunks_query
        .iter()
        .filter_map(|trunk| {
            if let Trunk::InChipper(chippers) = trunk {
                Some(chippers)
            } else {
                None
            }
        })
        .flatten()
        .copied()
        .collect();
    for (chipper_entity, chipper, mut chipper_effect) in chippers_query.iter_mut() {
        let target_effect = if matches!(chipper, Chipper::Jammed) {
            ChipperEffect::Smoking
        } else if chipping_chippers.contains(&chipper_entity) {
            ChipperEffect::ChippingWood
        } else {
            ChipperEffect::NoEffect
        };
        if target_effect == *chipper_effect {
            continue;
        }
        *chipper_effect = target_effect;
        let mut cmd = commands.entity(chipper_entity);
        match *chipper_effect {
            ChipperEffect::NoEffect => {
                cmd.remove::<ParticleEffect>();
            }
            ChipperEffect::ChippingWood => {
                cmd.insert(ParticleEffect::new(
                    particle_effects_assets.chipping_wood.clone(),
                ));
            }
            ChipperEffect::Smoking => {
                cmd.insert(ParticleEffect::new(particle_effects_assets.smoke.clone()));
            }
        }
    }
}
