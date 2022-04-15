use bevy::prelude::*;
use bevy_hanabi::{
    AccelModifier, ColorOverLifetimeModifier, EffectAsset, Gradient, ParticleEffect,
    ParticleEffectBundle, PositionSphereModifier, ShapeDimension, SizeOverLifetimeModifier,
    Spawner,
};

use crate::global_types::ParticleEffectType;

pub struct ParticleEffectPlugin;

impl Plugin for ParticleEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_entities_who_need_particle_effects);
        app.add_system(move_particle_effects_to_targets);
    }
}

#[derive(Component, Debug)]
struct ActualParticleEffect {
    effect_type: ParticleEffectType,
    entity: Option<Entity>,
}

#[derive(Component, Debug)]
struct HasParticleEffect(ParticleEffectType);

fn handle_entities_who_need_particle_effects(
    mut commands: Commands,
    targets_query: Query<(Entity, &ParticleEffectType, Option<&HasParticleEffect>)>,
    mut effects_query: Query<(&mut ActualParticleEffect, &mut ParticleEffect)>,
    mut particle_effects_assets: ResMut<Assets<EffectAsset>>,
) {
    for (target_entity, target_effect_type, has_effect) in targets_query.iter() {
        if let Some(has_effect) = has_effect {
            if has_effect.0 == *target_effect_type {
                continue;
            }
        }
        if let Some((mut available_effect, mut effect)) =
            effects_query.iter_mut().find(|(actual_effect, _)| {
                actual_effect.effect_type == *target_effect_type && actual_effect.entity.is_none()
            })
        {
            if let Some(spawner) = effect.maybe_spawner() {
                spawner.set_active(true);
            } else {
                continue;
            }
            available_effect.entity = Some(target_entity);
        } else {
            let mut cmd = commands.spawn();
            cmd.insert_bundle(ParticleEffectBundle {
                effect: ParticleEffect::new(
                    particle_effects_assets.add(create_effect(*target_effect_type)),
                ),
                ..Default::default()
            });
            cmd.insert(ActualParticleEffect {
                effect_type: *target_effect_type,
                entity: Some(target_entity),
            });
        }
        commands
            .entity(target_entity)
            .insert(HasParticleEffect(*target_effect_type));
    }
}

fn move_particle_effects_to_targets(
    mut commands: Commands,
    mut effects_query: Query<(
        &mut Transform,
        &mut ActualParticleEffect,
        &mut ParticleEffect,
    )>,
    targets_query: Query<(&GlobalTransform, &ParticleEffectType)>,
) {
    for (mut effect_transform, mut actual_effect, mut effect) in effects_query.iter_mut() {
        let target_entity = if let Some(target_entity) = actual_effect.entity {
            target_entity
        } else {
            continue;
        };
        if let Ok((target_transform, target_effect)) = targets_query.get(target_entity) {
            if *target_effect == actual_effect.effect_type {
                *effect_transform = Transform::from(*target_transform);
            } else {
                if let Some(spawner) = effect.maybe_spawner() {
                    spawner.set_active(false);
                } else {
                    continue;
                }
                actual_effect.entity = None;
                commands.entity(target_entity).remove::<HasParticleEffect>();
            }
        } else {
            if let Some(spawner) = effect.maybe_spawner() {
                spawner.set_active(false);
            } else {
                continue;
            }
            actual_effect.entity = None;
        }
    }
}

fn create_effect(effect: ParticleEffectType) -> EffectAsset {
    match effect {
        ParticleEffectType::ChippingWood => EffectAsset {
            name: "ChippingWood".to_string(),
            capacity: 100,
            spawner: Spawner::rate(5.0.into()),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 0.4,
            dimension: ShapeDimension::Volume,
            speed: bevy_hanabi::Value::Uniform((1.0, 10.0)),
        })
        .update(AccelModifier {
            accel: Vec3::new(0., -9.8, 0.),
        })
        .render(ColorOverLifetimeModifier {
            gradient: {
                let mut gradient = Gradient::new();
                gradient.add_key(0.0, Vec4::new(0.44, 0.33, 0.23, 1.0));
                gradient.add_key(1.0, Vec4::new(0.44, 0.33, 0.23, 0.0));
                gradient
            },
        })
        .render(SizeOverLifetimeModifier {
            gradient: {
                let mut gradient = Gradient::new();
                gradient.add_key(0.0, Vec2::splat(0.2));
                gradient.add_key(1.0, Vec2::splat(0.1));
                gradient
            },
        }),
        ParticleEffectType::Smoke => EffectAsset {
            name: "Smoke".to_string(),
            capacity: 100,
            spawner: Spawner::rate(10.0.into()),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 0.4,
            dimension: ShapeDimension::Volume,
            speed: bevy_hanabi::Value::Uniform((0.0, 1.0)),
        })
        .update(AccelModifier {
            accel: Vec3::new(0., 1.0, 0.),
        })
        .render(ColorOverLifetimeModifier {
            gradient: {
                let mut gradient = Gradient::new();
                gradient.add_key(0.0, Vec4::new(0.4, 0.4, 0.4, 0.5));
                gradient.add_key(1.0, Vec4::new(0.4, 0.4, 0.4, 0.3));
                gradient
            },
        })
        .render(SizeOverLifetimeModifier {
            gradient: {
                let mut gradient = Gradient::new();
                gradient.add_key(0.0, Vec2::splat(0.3));
                gradient.add_key(1.0, Vec2::splat(0.1));
                gradient
            },
        }),
        ParticleEffectType::Blood => EffectAsset {
            name: "Blood".to_string(),
            capacity: 100,
            spawner: Spawner::rate(30.0.into()),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 0.4,
            dimension: ShapeDimension::Volume,
            speed: bevy_hanabi::Value::Uniform((0.0, 3.0)),
        })
        .update(AccelModifier {
            accel: Vec3::new(0., -9.8, 0.),
        })
        .render(ColorOverLifetimeModifier {
            gradient: {
                let mut gradient = Gradient::new();
                gradient.add_key(0.0, Vec4::new(0.5412, 0.0118, 0.0118, 1.0));
                gradient.add_key(0.9, Vec4::new(0.5412, 0.0118, 0.0118, 1.0));
                gradient.add_key(1.0, Vec4::new(0.5412, 0.0118, 0.0118, 0.0));
                gradient
            },
        })
        .render(SizeOverLifetimeModifier {
            gradient: {
                let mut gradient = Gradient::new();
                gradient.add_key(0.0, Vec2::splat(0.2));
                gradient.add_key(1.0, Vec2::splat(0.1));
                gradient
            },
        }),
    }
}
