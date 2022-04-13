use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, Chipper, DespawnWithLevel, SpawnsWoodchips, Woodchip};
use crate::gltf_spawner::{SpawnCollider, SpawnGltfNode};
use crate::loading::ModelAssets;

pub struct WoodshipsPlugin;

impl Plugin for WoodshipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_woodchips));
    }
}

fn spawn_woodchips(
    mut commands: Commands,
    time: Res<Time>,
    model_assets: Res<ModelAssets>,
    mut spawners_query: Query<(Entity, &RigidBodyPositionComponent, &mut SpawnsWoodchips)>,
    narrow_phase: Res<NarrowPhase>,
    chipper_query: Query<&Chipper>,
) {
    for (spawner_entity, spawner_position, mut spawner) in spawners_query.iter_mut() {
        if spawner.0.tick(time.delta()).just_finished() {
            if !spawner.0.duration().is_zero() {
                let spawner_handle = spawner_entity.handle();
                let has_contact_with_chipper =
                    narrow_phase.contacts_with(spawner_handle).any(|contact| {
                        if !contact.has_any_active_contact {
                            return false;
                        }
                        let other_entity = if contact.collider1 == spawner_handle {
                            contact.collider2
                        } else {
                            contact.collider1
                        }
                        .entity();
                        chipper_query.get(other_entity).is_ok()
                    });
                if !has_contact_with_chipper {
                    commands.entity(spawner_entity).despawn_recursive();
                    continue;
                }

                let spawn_from_position = {
                    let pos1 = spawner_position.0.position * point![-1.0, 0.0];
                    let pos2 = spawner_position.0.position * point![1.0, 0.0];
                    if pos1.y < pos2.y {
                        pos2
                    } else {
                        pos1
                    }
                };
                let spawn_direction =
                    spawn_from_position - spawner_position.0.position * point![0.0, 0.0];
                let trunk_direction = spawner_position.0.position.rotation * vector![0.0, 1.0];
                let slope = trunk_direction.dot(&vector![0.0, 1.0]);
                let spawn_from_position =
                    spawn_from_position + trunk_direction / slope * (0.5 - spawn_from_position.y);
                let mut cmd = commands.spawn();
                cmd.insert_bundle(RigidBodyBundle {
                    body_type: RigidBodyType::Dynamic.into(),
                    mass_properties: RigidBodyMassProps {
                        local_mprops: MassProperties {
                            local_com: point![0.0, 0.0],
                            inv_mass: 1.0 / 30.0,
                            inv_principal_inertia_sqrt: 1.0 / 3.0,
                        },
                        ..Default::default()
                    }
                    .into(),
                    position: Isometry {
                        translation: spawn_from_position.into(),
                        rotation: spawner_position.0.position.rotation,
                    }
                    .into(),
                    velocity: RigidBodyVelocity {
                        linvel: {
                            let y_velovity = 5.0 + 7.0 * rand::random::<f32>();
                            let x_velovity = 3.0 + 2.0 * rand::random::<f32>();
                            vector![x_velovity * spawn_direction.x, y_velovity]
                        },
                        angvel: 10.0 * (rand::random::<f32>() - 0.5),
                    }
                    .into(),
                    ..Default::default()
                });
                cmd.insert(RigidBodyPositionSync::Discrete);
                cmd.insert(SpawnCollider {
                    gltf: model_assets.woodchip.clone(),
                    node_name: "Collider",
                    material: ColliderMaterial {
                        // friction: 2.0,
                        // restitution: todo!(),
                        // friction_combine_rule: todo!(),
                        // restitution_combine_rule: todo!(),
                        ..Default::default()
                    },
                    flags: Default::default(),
                });
                cmd.insert(Transform::from_xyz(0.0, 0.0, 0.0));
                cmd.insert(GlobalTransform::identity());
                cmd.insert(SpawnGltfNode(model_assets.woodchip.clone(), "Woodchip"));
                cmd.insert(Woodchip);
                cmd.insert(DespawnWithLevel);
            }
            let next_chip_in = 1.0 + 5.0 * rand::random::<f32>();
            spawner
                .0
                .set_duration(Duration::from_secs_f32(next_chip_in));
            spawner.0.reset();
        }
    }
}
