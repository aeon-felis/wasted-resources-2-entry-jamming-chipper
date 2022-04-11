use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, Chipper, DespawnWithLevel, Trunk};
use crate::gltf_spawner::{SpawnCollider, SpawnGltfNode};
use crate::loading::ModelAssets;
use crate::utils::entities_ordered_by_type;

pub struct TrunksPlugin;

impl Plugin for TrunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_trunk)
                .with_system(handle_trunk_hitting_chipper)
                .with_system(move_kinematic_trunks)
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
        mass_properties: RigidBodyMassProps {
            //flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
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
    cmd.insert(Trunk);
    cmd.insert(DespawnWithLevel);
}

fn handle_trunk_hitting_chipper(
    mut reader: EventReader<ContactEvent>,
    mut trunks_query: Query<&mut RigidBodyTypeComponent, With<Trunk>>,
    chippers_query: Query<(), With<Chipper>>,
) {
    for event in reader.iter() {
        if let ContactEvent::Started(handle1, handle2) = event {
            if let Some([trunk_entity, _chipper_entity]) = entities_ordered_by_type!(
                [handle1.entity(), handle2.entity()],
                trunks_query,
                chippers_query,
            ) {
                if let Ok(mut trunk_rigid_body_type) = trunks_query.get_mut(trunk_entity) {
                    trunk_rigid_body_type.0 = RigidBodyType::KinematicVelocityBased;
                }
            }
        }
    }
}

fn move_kinematic_trunks(
    mut trunks: Query<(&RigidBodyTypeComponent, &mut RigidBodyVelocityComponent), With<Trunk>>,
) {
    for (rigid_body_type, mut velocity) in trunks.iter_mut() {
        if rigid_body_type.0 != RigidBodyType::KinematicVelocityBased {
            continue;
        }
        velocity.0.linvel = vector![0.0, -1.0];
    }
}
