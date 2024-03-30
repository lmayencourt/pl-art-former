/* SPDX-License-Identifier: MIT
 *
 * This files uses concept from code:
 * https://github.com/mixandjam/Celeste-Movement/blob/master/Assets/Scripts/Movement.cs
 * Copyright (c) 2019 André Cardoso
 *
 * Copyright (c) 2024 Louis Mayencourt
 */

// Translate the controller inputs into movement for the player.

/* Inspiration from Celeste moves set
 * - https://www.youtube.com/watch?v=yorTG9at90g
 * - https://www.youtube.com/watch?v=STyY26a_dPY
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//  use crate::physics::{CollideEvent, CollideWith};
use crate::player::*;

const MAX_RUNNING_SPEED: f32 = 300.0;
// Force to apply to reach MAX_RUNNING_SPEED in 2 secs
const RUNNING_FORCE: f32 = PLAYER_MASS / 2.0 * 10.0 * MAX_RUNNING_SPEED;

const JUMP_SPEED: f32 = 600.0;
const MAX_FALLING_SPEED: f32 = 600.0;

pub fn player_movement(
    mut query: Query<(&Controller, &mut Player)>,
    sense_query: Query<(&Grounded, &EdgeGrab), With<Player>>,
    mut modifier_query: Query<(&mut ExternalForce, &mut Velocity, &mut GravityScale), With<Player>>,
) {
    let (controller, mut player) = query.single_mut();
    let (grounded, edge_grab) = sense_query.single();
    let (mut force, mut velocity, mut gravity_scale) = modifier_query.single_mut();

    let grounded = grounded.0;
    if !grounded && !edge_grab.0 {
        player.state = PlayerState::InAir;
    }

    // info!("Player state {:?}", player.state);
    // info!("Control state {:?}", controller.direction);
    // info!("Velocity {:?}", velocity);

    force.force = Vec2::ZERO;

    match player.state {
        PlayerState::Idle => {
            if controller.direction.x != 0.0 {
                player.state = PlayerState::Running;
            }

            if controller.action == Action::Jump {
                jump(&controller, &mut velocity);
            }
        }
        PlayerState::Walking => {}
        PlayerState::Running => {
            if controller.direction.x != 0.0 {
                apply_horizontal_force(&controller, &mut force, &mut velocity);
            } else if controller.action == Action::None {
                stop_horizontal_velocity(&mut velocity, &mut force, RUNNING_FORCE * 2.0);

                if velocity.linvel.x < 20.0 && velocity.linvel.x > -20.0 {
                    player.state = PlayerState::Idle;
                }
            }

            if controller.action == Action::Jump {
                jump(&controller, &mut velocity);
            }
        }
        PlayerState::InAir => {
            // Keep X movement control
            if controller.direction.x != 0.0 || controller.action == Action::Jump {
                apply_horizontal_force(&controller, &mut force, &mut velocity);
            } else if controller.action == Action::None {
                stop_horizontal_velocity(&mut velocity, &mut force, RUNNING_FORCE);
            }

            if controller.action != Action::Jump {
                stop_vertical_velocity(&mut velocity, &mut force, JUMP_SPEED);
            }

            // // Modify gravity according to Y velocity
            // if velocity.linvel.y > 10.0 {
            //     gravity_scale.0 = 8.0;
            // } else {
            //     gravity_scale.0 = 8.0;
            // }

            if velocity.linvel.y < -MAX_FALLING_SPEED {
                velocity.linvel.y = -MAX_FALLING_SPEED;
            }

            if grounded {
                if controller.action == Action::None {
                    player.state = PlayerState::Idle;
                } else if controller.direction.x != 0.0 {
                    player.state = PlayerState::Running;
                } else if controller.action == Action::Jump {
                    jump(&controller, &mut velocity);
                }
            }

            if edge_grab.0 {
                if controller.direction.x == player.facing_direction.x {
                    player.state = PlayerState::OnEdge;
                }
            }
        },
        PlayerState::OnEdge => {
            velocity.linvel = Vec2::ZERO;
            gravity_scale.0 = 0.0;
            if controller.action == Action::Jump {
                // Wall jump
                player.state = PlayerState::InAir;
                jump(&controller, &mut velocity);
                gravity_scale.0 = 16.0;
            }

            if controller.direction.x == -player.facing_direction.x {
                // Let go
                player.state = PlayerState::InAir;
                gravity_scale.0 = 16.0;
            }
        },
    }
}

fn apply_horizontal_force(
    controller: &Controller,
    force: &mut ExternalForce,
    velocity: &mut Velocity,
) {
    force.force.x = controller.direction.x * RUNNING_FORCE;

    if velocity.linvel.x > MAX_RUNNING_SPEED {
        velocity.linvel.x = MAX_RUNNING_SPEED;
    }
    if velocity.linvel.x < -MAX_RUNNING_SPEED {
        velocity.linvel.x = -MAX_RUNNING_SPEED;
    }
}

fn stop_horizontal_velocity(velocity: &mut Velocity, force: &mut ExternalForce, max_speed: f32) {
    if velocity.linvel.x > 20.0 {
        // apply opposing-force to stop movement
        force.force = Vec2::NEG_X * max_speed;
    } else if velocity.linvel.x < -20.0 {
        // apply opposing-force to stop movement
        force.force = Vec2::X * max_speed;
    }
}

fn jump(controller: &Controller, velocity: &mut Velocity) {
    velocity.linvel.y = controller.direction.y * JUMP_SPEED;
}

fn stop_vertical_velocity(velocity: &mut Velocity, force: &mut ExternalForce, max_speed: f32) {
    if velocity.linvel.y > 0.0 {
        force.force += Vec2::NEG_Y * max_speed * 200.0;
    }
}
