use bevy::prelude::shape;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ezinput::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel, InputBinding, PlayerControlled};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_player));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(player_control));
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
        }
        .into(),
        position: point![0.0, 2.0].into(),
        // damping: RigidBodyDamping {
        // linear_damping: 1.0,
        // angular_damping: 0.0,
        // }
        // .into(),
        ..Default::default()
    });
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::capsule(point![0.0, -0.5], point![0.0, 0.5], 0.5).into(),
        material: ColliderMaterial {
            friction: 2.0,
            // restitution: todo!(),
            // friction_combine_rule: todo!(),
            // restitution_combine_rule: todo!(),
            ..Default::default()
        }
        .into(),
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
    cmd.insert(JumpingPower {
        full_power: 30.0,
        full_jump_duration: 0.1,
        current: 0.0,
    });
    cmd.insert(DespawnWithLevel);
}

#[derive(Component)]
struct JumpingPower {
    full_power: f32,
    full_jump_duration: f32,
    current: f32,
}

fn player_control(
    time: Res<Time>,
    input_views: Query<&InputView<InputBinding>>,
    mut query: Query<
        (
            Entity,
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
            &mut JumpingPower,
        ),
        With<PlayerControlled>,
    >,
    narrow_phase: Res<NarrowPhase>,
) {
    let mut movement_value = 0.0;
    let mut num_participating = 0;
    let mut is_jumping = false;
    for input_view in input_views.iter() {
        for axis_value in input_view.axis(&InputBinding::MoveHorizontal) {
            if !axis_value.1.released() {
                num_participating += 1;
                movement_value = axis_value.0
            }
        }
        if matches!(input_view.key(&InputBinding::Jump), PressState::Pressed { .. }) {
            is_jumping = true;
        }
    }
    let movement_value = if 0 < num_participating {
        movement_value / num_participating as f32
    } else {
        0.0
    };
    let target_speed = movement_value;
    for (player_entity, mut velocity, mass_props, mut juming_power) in query.iter_mut() {
        let standing_on = narrow_phase.contacts_with(player_entity.handle())
            .filter(|contact| contact.has_any_active_contact)
            .flat_map(|contact| {
                contact.manifolds.iter().filter_map(|contact_manifold| {
                    let player_handle = player_entity.handle();
                    if contact_manifold.data.rigid_body1 == Some(player_handle) {
                        Some(-contact_manifold.data.normal)
                    } else if contact_manifold.data.rigid_body2 == Some(player_handle) {
                        Some(contact_manifold.data.normal)
                    } else {
                        None
                    }
                })
            })
            .max_by_key(|normal| float_ord::FloatOrd(normal.dot(&vector![0.0, 1.0])));
        if let Some(standing_on) = standing_on {
            let refill_percentage = standing_on.dot(&vector![0.0, 1.0]);
            if juming_power.current < refill_percentage {
                juming_power.current = refill_percentage;
            }
        } else if !is_jumping {
            juming_power.current = 0.0;
        }
        if is_jumping {
            let to_deplete = juming_power.current.min(time.delta().as_secs_f32() / juming_power.full_jump_duration);
            if 0.0 < to_deplete {
                juming_power.current -= to_deplete;
                velocity.apply_impulse(
                    mass_props,
                    vector![0.0, 1.0] * juming_power.full_power * to_deplete,
                );
            }
        }

        let current_speed = velocity.linvel.dot(&vector![1.0, 0.0]) / 20.0;
        if 0.0 < target_speed && target_speed <= current_speed {
            continue;
        } else if target_speed < 0.0 && current_speed <= target_speed {
            continue;
        }
        let impulse = target_speed - current_speed;
        let impulse = impulse.signum() * impulse.powi(4);
        velocity.apply_impulse(
            mass_props,
            vector![1.0, 0.0] * time.delta().as_secs_f32() * 1000.0 * impulse,
        );
    }
}