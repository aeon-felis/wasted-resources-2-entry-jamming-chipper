use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ezinput::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel, InputBinding, PlayerControlled};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_player));
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(player_control)
        });
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
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            local_mprops: MassProperties {
                local_com: point![0.0, 0.0],
                inv_mass: 1.0,
                inv_principal_inertia_sqrt: 0.0,
            },
            ..Default::default()
        }.into(),
        position: point![0.0, 2.0].into(),
        damping: RigidBodyDamping {
            linear_damping: 1.0,
            angular_damping: 0.0,
        }.into(),
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
    });
    cmd.insert(PlayerControlled);
    cmd.insert(DespawnWithLevel);
}

fn player_control(
    time: Res<Time>,
    input_views: Query<&InputView<InputBinding>>,
    mut query: Query<
        (
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        With<PlayerControlled>,
    >,
) {
    let mut movement_value = 0.0;
    let mut num_participating = 0;
    for input_view in input_views.iter() {
        for axis_value in input_view.axis(&InputBinding::MoveHorizontal) {
            if !axis_value.1.released() {
                num_participating += 1;
                movement_value = axis_value.0
            }
        }
    }
    if 0 < num_participating {
        movement_value /= num_participating as f32;
        let impulse = time.delta().as_secs_f32() * 100.0 * movement_value;
        for (mut velocity, mass_props) in query.iter_mut() {
            velocity.apply_impulse(mass_props, vector![1.0, 0.0] * impulse);
        }
    }
}
