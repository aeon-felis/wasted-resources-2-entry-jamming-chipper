use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_arena));
    }
}

fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut meterials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static.into(),
        position: point![0.0, -0.5].into(),
        ..Default::default()
    });
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(5.0, 0.5).into(),
        ..Default::default()
    });
    cmd.insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 2.0))),
        material: meterials.add(Color::BEIGE.into()),
        ..Default::default()
    })
    .insert(DespawnWithLevel);

    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static.into(),
        position: Isometry::new(vector![10.0, 0.0], 0.5).into(),
        ..Default::default()
    });
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(5.0, 0.5).into(),
        ..Default::default()
    });
    cmd.insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 2.0))),
        material: meterials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..Default::default()
    })
    .insert(DespawnWithLevel);
}
