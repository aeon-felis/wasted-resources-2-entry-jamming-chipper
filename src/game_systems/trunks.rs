use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, Chipper, DespawnWithLevel, SpawnsWoodchips, Trunk};
use crate::gltf_spawner::{SpawnCollider, SpawnGltfNode};
use crate::loading::ModelAssets;
use crate::utils::{entities_ordered_by_type, ok_or, some_or};

pub struct TrunksPlugin;

impl Plugin for TrunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_trunk)
                .with_system(handle_trunk_hitting_chipper)
                .with_system(chippers_resist_trunk)
                .with_system(handle_lost_trunks)
        });
    }
}

fn spawn_trunk(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    current_logs: Query<(), With<Trunk>>,
) {
    if current_logs.iter().next().is_some() {
        return;
    }
    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        // body_type: RigidBodyType::KinematicVelocityBased.into(),
        mass_properties: RigidBodyMassProps {
            local_mprops: MassProperties {
                local_com: point![0.0, 0.0],
                inv_mass: 1.0 / 3000.0,
                inv_principal_inertia_sqrt: 1.0 / 300.0,
            },
            ..Default::default()
        }
        .into(),
        position: point![10.0, 5.0].into(),
        velocity: RigidBodyVelocity {
            linvel: vector![-6.0, 3.0],
            angvel: -0.5,
        }
        .into(),
        // damping: RigidBodyDamping {
        // linear_damping: 1.0,
        // angular_damping: 0.0,
        // }
        // .into(),
        ..Default::default()
    });
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert(SpawnCollider {
        gltf: model_assets.trunk.clone(),
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
    cmd.insert(Transform::from_xyz(0.0, 2.0, 0.0));
    cmd.insert(GlobalTransform::identity());
    cmd.insert(SpawnGltfNode(model_assets.trunk.clone(), "Trunk"));
    cmd.insert(Trunk::Free);
    cmd.insert(DespawnWithLevel);
}

fn handle_trunk_hitting_chipper(
    mut reader: EventReader<IntersectionEvent>,
    mut trunks_query: Query<(&mut Trunk, &mut RigidBodyTypeComponent)>,
    chippers_query: Query<&Chipper>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        let [trunk_entity, chipper_entity] = some_or!(entities_ordered_by_type!(
                [event.collider1.entity(), event.collider2.entity()],
                trunks_query,
                chippers_query,
        ); continue);
        let (mut trunk, mut _trunk_rigid_body_type) =
            ok_or!(trunks_query.get_mut(trunk_entity); continue);
        if event.intersecting {
            match &mut *trunk {
                Trunk::Free => {
                    // trunk_rigid_body_type.0 = RigidBodyType::KinematicVelocityBased;
                    *trunk = Trunk::InChipper([chipper_entity].into_iter().collect());
                    commands
                        .entity(trunk_entity)
                        .insert(SpawnsWoodchips(Timer::new(Duration::ZERO, false)));
                }
                Trunk::InChipper(trunk_chippers) => {
                    trunk_chippers.insert(chipper_entity);
                }
            }
        } else if let Trunk::InChipper(trunk_chippers) = &mut *trunk {
            if trunk_chippers.contains(&chipper_entity) {
                trunk_chippers.remove(&chipper_entity);
                if trunk_chippers.is_empty() {
                    commands.entity(trunk_entity).despawn_recursive();
                }
            }
        }
    }
}

fn handle_lost_trunks(
    mut commands: Commands,
    trunks: Query<(Entity, &Trunk, &RigidBodyPositionComponent)>,
) {
    for (trunk_entity, trunk, trunk_position) in trunks.iter() {
        if matches!(trunk, Trunk::Free) && trunk_position.position.translation.y < -8.0 {
            commands.entity(trunk_entity).despawn_recursive();
        }
    }
}

fn chippers_resist_trunk(
    mut trunks_query: Query<(&Trunk, &mut RigidBodyVelocityComponent)>,
    chippers_query: Query<&Chipper>,
) {
    for (trunk, mut trunk_velocity) in trunks_query.iter_mut() {
        if let Trunk::InChipper(chippers) = trunk {
            if chippers.is_empty() {
                continue;
            }
            if chippers.iter().any(|chipper_entity| {
                matches!(chippers_query.get(*chipper_entity), Ok(Chipper::Jammed))
            }) {
                trunk_velocity.angvel = 0.0;
                trunk_velocity.linvel = vector![0.0, 0.0];
            } else {
                trunk_velocity.angvel = 0.0;
                trunk_velocity.linvel = vector![0.0, -1.0];
            }
        }
    }
}
