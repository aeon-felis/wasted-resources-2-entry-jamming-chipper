use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_player));
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut meterials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        mass_properties: MassProperties {
            local_com: point![0.0, 0.0],
            inv_mass: 1.0,
            inv_principal_inertia_sqrt: 0.0,
        }
        .into(),
        position: point![0.0, 2.0].into(),
        // damping: todo!(),
        ..Default::default()
    });
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::capsule(point![0.0, -0.5], point![0.0, 0.5], 0.5).into(),
        // material: todo!(),
        ..Default::default()
    });
    cmd.insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5,
            rings: 10,
            depth: 1.0,
            latitudes: 10,
            longitudes: 10,
            uv_profile: shape::CapsuleUvProfile::Uniform,
        })),
        material: meterials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    })
    .insert(DespawnWithLevel);
}
