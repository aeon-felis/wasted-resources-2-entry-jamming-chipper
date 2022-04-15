use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::lens::TransformRotateYLens;
use bevy_tweening::{Animator, AnimatorState, EaseFunction, Lens, Tween, TweeningType};
use ezinput::prelude::*;

use crate::global_types::{
    AppState, Chipper, DespawnWithLevel, InputBinding, MenuState, ParticleEffectType, PlayerControl,
};
use crate::gltf_spawner::{GltfNodeAddedEvent, SpawnCollider, SpawnGltfNode};
use crate::loading::ModelAssets;
use crate::utils::{entities_ordered_by_type, ok_or, some_or};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_player));
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(player_control)
                .with_system(add_animation)
                .with_system(player_animation)
                .with_system(kill_player)
                .with_system(game_over_when_player_falls_too_much)
        });
    }
}

#[derive(Component, Clone)]
struct PlayerStatusForAnimation {
    is_moving: bool,
    was_moving: bool,
    is_left: bool,
    was_left: bool,
    body_entity: Entity,
    leg_entities: [Entity; 2],
}

fn setup_player(mut commands: Commands, model_assets: Res<ModelAssets>) {
    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            local_mprops: MassProperties {
                local_com: point![0.0, 0.0],
                inv_mass: 1.0 / 80.0,
                inv_principal_inertia_sqrt: 0.0,
            },
            ..Default::default()
        }
        .into(),
        position: point![-3.0, 12.0].into(),
        // damping: RigidBodyDamping {
        // linear_damping: 1.0,
        // angular_damping: 0.0,
        // }
        // .into(),
        ..Default::default()
    });
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert(SpawnCollider {
        gltf: model_assets.player.clone(),
        node_name: "Collider",
        collider_type: ColliderType::Solid,
        material: ColliderMaterial {
            friction: 4.0,
            // restitution: todo!(),
            // friction_combine_rule: todo!(),
            // restitution_combine_rule: todo!(),
            ..Default::default()
        },
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
    });
    //cmd.insert_bundle(ColliderBundle {
    //shape: ColliderShape::capsule(point![0.0, -0.5], point![0.0, 0.5], 0.5).into(),
    //material: ColliderMaterial {
    //friction: 2.0,
    //// restitution: todo!(),
    //// friction_combine_rule: todo!(),
    //// restitution_combine_rule: todo!(),
    //..Default::default()
    //}
    //.into(),
    //..Default::default()
    //});
    // cmd.insert_bundle(PbrBundle {
    // //mesh: meshes.add(Mesh::from(shape::Capsule {
    // //radius: 0.5,
    // //rings: 10,
    // //depth: 1.0,
    // //latitudes: 10,
    // //longitudes: 10,
    // //uv_profile: shape::CapsuleUvProfile::Uniform,
    // //})),
    // //material: meterials.add(Color::RED.into()),
    // transform: Transform::from_xyz(0.0, 2.0, 0.0),
    // ..Default::default()
    // });
    cmd.insert(Transform::from_xyz(0.0, 2.0, 0.0));
    cmd.insert(GlobalTransform::identity());
    cmd.insert(Visibility::default());
    cmd.insert(ComputedVisibility::default());
    let mut body_entity = None;
    let mut leg_entities = Vec::new();
    cmd.with_children(|commands| {
        body_entity = Some(
            commands
                .spawn_bundle((
                    GlobalTransform::identity(),
                    Transform::identity(),
                    SpawnGltfNode(model_assets.player.clone(), "Body"),
                    Animator::<Transform>::default(),
                ))
                .with_children(|commands| {
                    for (node_name, leg_type) in
                        [("RightLeg", PlayerLeg::Right), ("LeftLeg", PlayerLeg::Left)]
                    {
                        leg_entities.push({
                            commands
                                .spawn()
                                .insert(Transform::identity())
                                .insert(GlobalTransform::identity())
                                .insert(SpawnGltfNode(model_assets.player.clone(), node_name))
                                .insert(Animator::<Transform>::default())
                                .insert(leg_type)
                                .id()
                        });
                    }
                })
                .id(),
        );
    });
    cmd.insert(PlayerControl {
        max_speed: 20.0,
        impulse_coefficient: 40_000.0,
        jump_power_coefficient: 800.0,
        jump_from_woodchip_power_coefficient: 200.0,
        jump_time_coefficient: 7.5,
        jump_potential: 0.0,
        last_stood_on: vector![0.0, 1.0],
        stood_on_potential: 0.0,
        stood_on_time_coefficient: 10.0,
        uphill_move_efficiency: 0.5,
        uphill_stop_efficiency: 1.0,
    });
    cmd.insert(PlayerStatusForAnimation {
        is_moving: false,
        was_moving: false,
        is_left: false,
        was_left: false,
        body_entity: body_entity.unwrap(),
        leg_entities: leg_entities.try_into().unwrap(),
    });
    cmd.insert(IsPlayerAlive(true));
    cmd.insert(DespawnWithLevel);
}

#[derive(Component)]
enum PlayerLeg {
    Right,
    Left,
}

fn player_control(
    time: Res<Time>,
    input_views: Query<&InputView<InputBinding>>,
    mut query: Query<(
        Entity,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        &IsPlayerAlive,
        &mut PlayerControl,
        &mut PlayerStatusForAnimation,
    )>,
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
        if matches!(
            input_view.key(&InputBinding::Jump),
            PressState::Pressed { .. }
        ) {
            is_jumping = true;
        }
    }
    let movement_value = if 0 < num_participating {
        movement_value / num_participating as f32
    } else {
        0.0
    };
    let target_speed = movement_value;
    for (
        player_entity,
        mut velocity,
        mass_props,
        is_player_alive,
        mut player_control,
        mut player_status_for_animation,
    ) in query.iter_mut()
    {
        if !is_player_alive.0 {
            continue;
        }
        let standing_on = narrow_phase
            .contacts_with(player_entity.handle())
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
            if player_control.jump_potential < refill_percentage {
                player_control.jump_potential = refill_percentage;
            }

            player_control.last_stood_on = standing_on;
            player_control.stood_on_potential = 1.0;
        } else {
            if !is_jumping {
                player_control.jump_potential = 0.0;
            }

            player_control.stood_on_potential = (player_control.stood_on_potential
                - time.delta().as_secs_f32() * player_control.stood_on_time_coefficient)
                .max(0.0);
        }
        if is_jumping {
            let to_deplete = player_control
                .jump_potential
                .min(time.delta().as_secs_f32() * player_control.jump_time_coefficient);
            if 0.0 < to_deplete {
                let before_depletion = player_control.jump_potential;
                let after_depletion = before_depletion - to_deplete;
                player_control.jump_potential = after_depletion;
                let integrate = |x: f32| {
                    let degree = 0.75;
                    x.powf(degree) / degree
                };
                let area_under_graph =
                    (integrate(before_depletion) - integrate(after_depletion)) / integrate(1.0);
                velocity.apply_impulse(
                    mass_props,
                    vector![0.0, 1.0] * player_control.jump_power_coefficient * area_under_graph,
                );
            }
        }

        let mut up_now = vector![0.0, 1.0];
        up_now = (1.0 - player_control.stood_on_potential) * up_now
            + player_control.stood_on_potential * player_control.last_stood_on;

        let movement_vector = Isometry::rotation(-std::f32::consts::FRAC_PI_2) * up_now;

        let current_speed = velocity.linvel.dot(&movement_vector) / player_control.max_speed;
        player_status_for_animation.is_moving = 0.01 <= target_speed.abs();
        if player_status_for_animation.is_moving {
            player_status_for_animation.is_left = target_speed < 0.0;
        }

        if (0.0 < target_speed && target_speed <= current_speed)
            || (target_speed < 0.0 && current_speed <= target_speed)
        {
            continue;
        }
        let impulse = target_speed - current_speed;
        let impulse = if 1.0 < impulse.abs() {
            impulse.signum()
        } else {
            impulse.signum() * impulse.powi(4)
        };
        let mut impulse = movement_vector
            * time.delta().as_secs_f32()
            * player_control.impulse_coefficient
            * impulse;
        let uphill = impulse.normalize().dot(&vector![0.0, 1.0]);
        if 0.01 <= uphill {
            let efficiency = if target_speed.signum() as i32 == current_speed.signum() as i32 {
                player_control.uphill_move_efficiency
            } else {
                player_control.uphill_stop_efficiency
            };
            impulse *= 1.0 - uphill.powf(efficiency);
        }
        velocity.apply_impulse(mass_props, impulse);
    }
}

fn add_animation(
    mut event_reader: EventReader<GltfNodeAddedEvent>,
    mut legs_query: Query<(&PlayerLeg, &Transform, &mut Animator<Transform>)>,
) {
    for GltfNodeAddedEvent(entity) in event_reader.iter() {
        if let Ok((leg, transform, mut animator)) = legs_query.get_mut(*entity) {
            let base_transform = *transform;
            let translation = Vec3::new(0.0, 1.0, 0.0);
            const POSITIVE_ANGLE: f32 = 0.4;
            const NEGATIVE_ANGLE: f32 = -0.2;
            let (start, end) = match leg {
                PlayerLeg::Right => (POSITIVE_ANGLE, NEGATIVE_ANGLE),
                PlayerLeg::Left => (NEGATIVE_ANGLE, POSITIVE_ANGLE),
            };
            animator.set_tweenable(Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::PingPong,
                Duration::from_millis(200),
                LegLens {
                    base_transform,
                    translation,
                    start,
                    end,
                },
            ));
            animator.state = AnimatorState::Paused;
        }
    }
}

struct LegLens {
    base_transform: Transform,
    translation: Vec3,
    start: f32,
    end: f32,
}

impl Lens<Transform> for LegLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = ratio * (self.end - self.start) + self.start;
        *target = self.base_transform
            * Transform::from_translation(self.translation)
            * Transform::from_rotation(Quat::from_rotation_z(value))
            * Transform::from_translation(-self.translation);
    }
}

fn player_animation(
    mut statuses_for_animation: Query<&mut PlayerStatusForAnimation>,
    mut animators: Query<&mut Animator<Transform>>,
) {
    for mut status_for_animation in statuses_for_animation.iter_mut() {
        if status_for_animation.is_left != status_for_animation.was_left {
            status_for_animation.was_left = status_for_animation.is_left;
            let mut animator = animators.get_mut(status_for_animation.body_entity).unwrap();
            animator.set_tweenable(Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::Once,
                Duration::from_millis(100),
                if status_for_animation.is_left {
                    TransformRotateYLens {
                        start: 0.0,
                        end: std::f32::consts::PI,
                    }
                } else {
                    TransformRotateYLens {
                        start: std::f32::consts::PI,
                        end: 2.0 * std::f32::consts::PI,
                    }
                },
            ));
        }

        if status_for_animation.is_moving != status_for_animation.was_moving {
            status_for_animation.was_moving = status_for_animation.is_moving;
            for leg_entity in status_for_animation.leg_entities.iter() {
                let mut animator = animators.get_mut(*leg_entity).unwrap();
                animator.state = if status_for_animation.is_moving {
                    AnimatorState::Playing
                } else {
                    AnimatorState::Paused
                };
            }
        }
    }
}

#[derive(Component)]
struct IsPlayerAlive(bool);

fn kill_player(
    mut commands: Commands,
    mut reader: EventReader<IntersectionEvent>,
    mut players_query: Query<(
        &mut IsPlayerAlive,
        &mut RigidBodyDominanceComponent,
        &mut RigidBodyVelocityComponent,
    )>,
    chippers_query: Query<&Chipper>,
) {
    for event in reader.iter() {
        if !event.intersecting {
            continue;
        }
        let [player_entity, _chipper_entity] = some_or!(entities_ordered_by_type!(
                [event.collider1.entity(), event.collider2.entity()],
                players_query,
                chippers_query,
        ); continue);
        let (mut is_player_alive, mut player_dominance, mut player_velocity) =
            ok_or!(players_query.get_mut(player_entity); continue);
        if !is_player_alive.0 {
            continue;
        }
        is_player_alive.0 = false;
        player_dominance.0 = RigidBodyDominance(127);
        player_velocity.0.linvel = vector![0.0, 5.0];
        player_velocity.0.angvel = 10.0;
        commands
            .entity(player_entity)
            .insert(ParticleEffectType::Blood);
    }
}

fn game_over_when_player_falls_too_much(
    players_query: Query<&RigidBodyPositionComponent, With<IsPlayerAlive>>,
    mut state: ResMut<State<AppState>>,
) {
    for player_position in players_query.iter() {
        if player_position.position.translation.y < -4.0 {
            state.set(AppState::Menu(MenuState::GameOver)).unwrap();
        }
    }
}
