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

const MAX_RUNNING_SPEED: f32 = 250.0;
// Force to apply to reach MAX_RUNNING_SPEED in 2 secs
const RUNNING_FORCE: f32 = PLAYER_MASS / 2.0 * 20.0 * MAX_RUNNING_SPEED;

const JUMP_SPEED: f32 = 600.0;
const MAX_FALLING_SPEED: f32 = 600.0;
const MAX_WALL_SLIDING_SPEED: f32 = 200.0;

#[derive(Component, Deref, DerefMut)]
pub struct InhibitionTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CoyoteTimer(pub Timer);

pub fn player_movement(
    mut query: Query<(&Controller, &mut Player)>,
    sense_query: Query<(&Grounded, &OnWall, &EdgeGrab), With<Player>>,
    mut modifier_query: Query<(&mut ExternalForce, &mut Velocity, &mut GravityScale), With<Player>>,
    mut timer_query: Query<(&mut InhibitionTimer, &mut CoyoteTimer), With<Player>>,
    time: Res<Time>,
) {
    let (controller, mut player) = query.single_mut();
    let (grounded, on_wall, edge_grab) = sense_query.single();
    let (mut force, mut velocity, mut gravity_scale) = modifier_query.single_mut();
    let (mut inhibition_timer, mut coyote_timer) = timer_query.single_mut();

    let player_still: bool = velocity.linvel.x < 20.0 && velocity.linvel.x > -20.0 && controller.direction.x == 0.0;

    if grounded.0 && player_still {
        player.state = PlayerState::Idle;
    } else if grounded.0 && !player_still {
        player.state = PlayerState::Running;
    } else if !grounded.0 && !edge_grab.0 && !on_wall.0{
        player.state = PlayerState::InAir;
    } else if !grounded.0 && edge_grab.0 {
        if controller.direction.x == player.facing_direction.x {
            player.state = PlayerState::OnEdge;
        } else {
            player.state = PlayerState::OnWall;
        }
    } else if !grounded.0 && !edge_grab.0 && on_wall.0 {
        player.state = PlayerState::OnWall;
    }

    // info!("Player state {:?}", player.state);
    // info!("Control state {:?}", controller);
    // info!("Velocity {:?}", velocity);
    // info!("Sensing grounded {} on_wall {}", grounded.0, on_wall.0);

    force.force = Vec2::ZERO;

    // Prevent movement control for a given amount of time
    inhibition_timer.tick(time.delta());
    if !inhibition_timer.finished() {
        return;
    }

    // Apply action according to player state
    match player.state {
        PlayerState::Idle => {
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
            }

            if controller.action == Action::Jump {
                jump(&controller, &mut velocity);
            }
        }
        PlayerState::InAir => {
            gravity_scale.0 = 16.0;

            // Keep X movement control
            if controller.direction.x != 0.0 {
                apply_horizontal_force(&controller, &mut force, &mut velocity);
            } else if controller.action == Action::None {
                stop_horizontal_velocity(&mut velocity, &mut force, RUNNING_FORCE);
            }

            if controller.action != Action::Jump {
                stop_vertical_velocity(&mut velocity, &mut force, JUMP_SPEED);
            }

            if velocity.linvel.y < -MAX_FALLING_SPEED {
                velocity.linvel.y = -MAX_FALLING_SPEED;
            }
        },
        PlayerState::OnEdge => {
            velocity.linvel = Vec2::ZERO;
            gravity_scale.0 = 0.0;
            
            if controller.action == Action::Jump {
                jump(&controller, &mut velocity);
                // coyote_timer.reset();
            }
        },
        PlayerState::OnWall => {
            gravity_scale.0 = 16.0;
            if velocity.linvel.y < -MAX_WALL_SLIDING_SPEED {
                velocity.linvel.y = -MAX_WALL_SLIDING_SPEED;
            }

            if controller.action == Action::Jump {
                wall_jump(controller.jump_released, &player.facing_direction, &mut velocity);
                inhibition_timer.reset();
            }
        }
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
    if controller.jump_released {
        velocity.linvel.y = controller.direction.y * JUMP_SPEED;
    }
}

fn wall_jump(can_jump: bool, direction: &Vec2, velocity: &mut Velocity) {
    if can_jump {
        velocity.linvel.y = JUMP_SPEED;
        velocity.linvel.x = -direction.x * MAX_RUNNING_SPEED;
    }
}

fn stop_vertical_velocity(velocity: &mut Velocity, force: &mut ExternalForce, max_speed: f32) {
    if velocity.linvel.y > 0.0 {
        force.force += Vec2::NEG_Y * max_speed * 200.0;
    }
}
