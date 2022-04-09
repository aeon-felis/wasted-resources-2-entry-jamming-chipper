use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel, Trunk};
use crate::gltf_spawner::{SpawnCollider, SpawnGltfNode};
use crate::loading::ModelAssets;

pub struct TrunksPlugin;

impl Plugin for TrunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_trunk)
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
                inv_mass: 1.0 / 300.0,
                inv_principal_inertia_sqrt: 1.0 / 30.0,
            },
            ..Default::default()
        }
        .into(),
        position: point![10.0, 5.0].into(),
        velocity: RigidBodyVelocity {
            linvel: vector![-5.0, 3.0],
            angvel: -1.0,
        }.into(),
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
    });
    cmd.insert(Transform::from_xyz(0.0, 2.0, 0.0));
    cmd.insert(GlobalTransform::identity());
    cmd.insert(SpawnGltfNode(model_assets.trunk.clone(), "Trunk"));
    cmd.insert(Trunk);
    cmd.insert(DespawnWithLevel);
}
