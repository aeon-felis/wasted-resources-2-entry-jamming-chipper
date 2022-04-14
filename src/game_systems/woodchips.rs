use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{Animator, AnimatorState};

use crate::global_types::{
    AppState, Chipper, DespawnWithLevel, PlayerControl, SpawnsWoodchips, Woodchip,
};
use crate::gltf_spawner::{SpawnCollider, SpawnGltfNode};
use crate::loading::ModelAssets;
use crate::utils::{entities_ordered_by_type, ok_or, some_or};

pub struct WoodshipsPlugin;

impl Plugin for WoodshipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_woodchips)
                .with_system(handle_chip_hitting_chipper)
                .with_system(handle_player_jump_from_chipper)
        });
    }
}

fn spawn_woodchips(
    mut commands: Commands,
    time: Res<Time>,
    model_assets: Res<ModelAssets>,
    mut spawners_query: Query<(&RigidBodyPositionComponent, &mut SpawnsWoodchips)>,
) {
    for (spawner_position, mut spawner) in spawners_query.iter_mut() {
        if spawner.0.tick(time.delta()).just_finished() {
            if !spawner.0.duration().is_zero() {
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
                    collider_type: ColliderType::Solid,
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
                cmd.insert(Woodchip::Free);
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

fn handle_chip_hitting_chipper(
    mut reader: EventReader<IntersectionEvent>,
    mut woodchips_query: Query<(
        &RigidBodyPositionComponent,
        &mut RigidBodyTypeComponent,
        &mut Woodchip,
    )>,
    mut chippers_query: Query<(&mut Chipper, &Children)>,
    mut saws_query: Query<&mut Animator<Transform>>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        let [woodchip_entity, chipper_entity] = some_or!(entities_ordered_by_type!(
                [event.collider1.entity(), event.collider2.entity()],
                woodchips_query,
                chippers_query,
        ); continue);
        if event.intersecting {
            let (mut chipper, chipper_children) =
                ok_or!(chippers_query.get_mut(chipper_entity); continue);
            if !matches!(*chipper, Chipper::Free) {
                continue;
            }
            let (woodchip_transform, mut woodchip_rigid_body_type, mut woodchip) =
                ok_or!(woodchips_query.get_mut(woodchip_entity); continue);
            if !matches!(*woodchip, Woodchip::Free) {
                continue;
            }
            if woodchip_transform.0.position.rotation.cos_angle().abs() < 0.5 {
                commands.entity(woodchip_entity).despawn_recursive();
            } else {
                *woodchip = Woodchip::StuckInChipper(chipper_entity);
                woodchip_rigid_body_type.0 = RigidBodyType::Static;
                *chipper = Chipper::Jammed;
                for saw_entity in chipper_children.iter() {
                    if let Ok(mut saw_animator) = saws_query.get_mut(*saw_entity) {
                        saw_animator.state = AnimatorState::Paused;
                    }
                }
            }
        }
    }
}

fn handle_player_jump_from_chipper(
    mut reader: EventReader<ContactEvent>,
    woodchips_query: Query<&Woodchip>,
    mut player_query: Query<(
        &mut PlayerControl,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
    mut chippers_query: Query<(&mut Chipper, &Children)>,
    mut saws_query: Query<&mut Animator<Transform>>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        if let ContactEvent::Started(handle1, handle2) = event {
            if let Some([woodchip_entity, player_entity]) = entities_ordered_by_type!(
                [handle1.entity(), handle2.entity()],
                woodchips_query,
                player_query,
            ) {
                let chipper_to_unjam = if let Ok(Woodchip::StuckInChipper(chipper)) =
                    woodchips_query.get(woodchip_entity)
                {
                    chipper
                } else {
                    continue;
                };
                let (mut player_control, mut player_velocity, player_mass_props) =
                    ok_or!(player_query.get_mut(player_entity); continue);
                if player_velocity.linvel.y <= 0.1 {
                    player_control.jump_potential = 1.0;
                    let compensate =
                        player_velocity.linvel.y / player_mass_props.local_mprops.inv_mass;
                    player_velocity.apply_impulse(
                        player_mass_props,
                        vector![0.0, 1.0]
                            * (player_control.jump_from_woodchip_power_coefficient - compensate),
                    );
                }

                let (mut chipper, chipper_children) =
                    ok_or!(chippers_query.get_mut(*chipper_to_unjam); continue);
                *chipper = Chipper::Free;
                for saw_entity in chipper_children.iter() {
                    if let Ok(mut saw_animator) = saws_query.get_mut(*saw_entity) {
                        saw_animator.state = AnimatorState::Playing;
                    }
                }

                commands.entity(woodchip_entity).despawn_recursive();
            }
        }
    }
}
