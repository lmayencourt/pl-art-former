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
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//  use crate::physics::{CollideEvent, CollideWith};
use crate::player::*;

const MAX_RUNNING_SPEED: f32 = 250.0;
// Force to apply to reach MAX_RUNNING_SPEED in 2 secs
const RUNNING_FORCE: f32 = PLAYER_MASS / 2.0 * 20.0 * MAX_RUNNING_SPEED;

const JUMP_SPEED: f32 = 600.0;
const MAX_FALLING_SPEED: f32 = 600.0;
const MAX_WALL_SLIDING_SPEED: f32 = 100.0;

// Define if the player can jump more than once before been grounded or on wall again
pub const PLAYER_MAX_JUMP_COUNT: u32 = 1;

#[derive(Component, Deref, DerefMut)]
pub struct InhibitionTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CoyoteTimer(pub Timer);

/// Inform other system that the player just performed a jump
#[derive(Event, Default)]
pub struct JustJumped;

/// Inform the coyote system that a late jump can be possible
#[derive(Event, Default)]
pub struct CoyoteStart(JumpedFrom);

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum JumpedFrom {
    #[default] Ground,
    WallOrEdge,
}

#[derive(Resource)]
pub struct CoyoteJumpedFrom {
    pub jumped_from: JumpedFrom,
}

#[derive(Resource)]
pub struct BufferedJump {
    pub should_jump: bool,
    pub timer: Timer,
}

pub fn player_movement(
    mut query: Query<(&Controller, &mut Player)>,
    sense_query: Query<(&Grounded, &OnWall, &EdgeGrab), With<Player>>,
    mut modifier_query: Query<(&mut ExternalForce, &mut Velocity, &mut GravityScale), With<Player>>,
    mut timer_query: Query<&mut InhibitionTimer, With<Player>>,
    mut action_event: EventReader<ActionEvent>,
    mut jump_event: EventWriter<JustJumped>,
    mut coyote_event: EventWriter<CoyoteStart>,
    mut buffured_jump: ResMut<BufferedJump>,
    time: Res<Time>,
) {
    let (controller, mut player) = query.single_mut();
    let (grounded, on_wall, edge_grab) = sense_query.single();
    let (mut force, mut velocity, mut gravity_scale) = modifier_query.single_mut();
    let mut inhibition_timer = timer_query.single_mut();

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
        if player.previous_state == PlayerState::Climbing {
            player.state = PlayerState::Climbing;
        } else {
            player.state = PlayerState::OnWall;
        }
    }

    // Define if player can jump
    if (grounded.0 || on_wall.0) && controller.jump_released {
        player.jump_count = 0;
    }
    player.can_jump = player.jump_count < PLAYER_MAX_JUMP_COUNT;

    for event in action_event.read() {
        if event.0 == Action::Jump {
            debug!("Start of jump event");
            buffured_jump.timer.reset();
        }

        match event.0 {
            Action::Jump => {
                debug!("Start of jump event");
                buffured_jump.timer.reset();
            },
            Action::EnterClimbingMode => {
                if on_wall.0 {
                    info!("Enter climbing mode");
                    player.state = PlayerState::Climbing;
                }
            },
            Action::ExitClimbingMode => {
                info!("Exit climbing mode");
                player.state = PlayerState::OnWall;
            },
            Action::GrabLeft => {
                info!("Grab left hold");
            },
            Action::GrabRight => {
                info!("Grab right hold");
            },
            Action::GrabUp => {
                info!("Grab up hold");
            },
            Action::GrabDown => {
                info!("Grab down hold");
            }
            _ => {}
        }
    }

    // Buffered jump
    // Allow the player to press the jump button slightly before being able to jump
    buffured_jump.timer.tick(time.delta());
    if !buffured_jump.timer.finished() {
        // info!("Apply buffered jump: {}", buffured_jump.should_jump);
        buffured_jump.should_jump = true; //buffured_jump.should_jump;
    } else {
        buffured_jump.should_jump = false;
        // buffured_jump.should_jump = false;
    }

    if buffured_jump.timer.just_finished() {
        debug!("End of jump event");
    }

    // Coyote time start
    // Allow the player to jump after leaving the ground or a wall
    if player.previous_state != PlayerState::InAir && player.state == PlayerState::InAir {
        if player.previous_state == PlayerState::OnEdge || player.previous_state == PlayerState::OnWall {
            coyote_event.send(CoyoteStart(JumpedFrom::WallOrEdge));
        } else {
            coyote_event.send_default();
        }
    }

    // info!("Player state {:?}", player.state);
    // info!("Control state {:?}", controller);
    // info!("Velocity {:?}", velocity);
    // info!("Sensing grounded: {}, on_wall {}, can jump {}, released {}", grounded.0, on_wall.0, player.jump_count, controller.jump_released);

    force.force = Vec2::ZERO;

    // Prevent movement control for a given amount of time
    inhibition_timer.tick(time.delta());
    if !inhibition_timer.finished() {
        return;
    }

    // Apply action according to player state
    match player.state {
        PlayerState::Idle => {
            if buffured_jump.should_jump {
                jump(&mut player, &controller, &mut velocity, &mut jump_event);
            }
        }
        PlayerState::Walking => {}
        PlayerState::Running => {
            if controller.direction.x != 0.0 {
                apply_horizontal_force(&controller, &mut force, &mut velocity);
            } else if controller.action == Action::None {
                stop_horizontal_velocity(&mut velocity, &mut force, RUNNING_FORCE * 2.0);
            }

            if buffured_jump.should_jump {
                jump(&mut player, &controller, &mut velocity, &mut jump_event);
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
            
            if buffured_jump.should_jump {
                jump(&mut player, &controller, &mut velocity, &mut jump_event);
                inhibition_timer.set_duration(Duration::from_millis(30));
                inhibition_timer.reset();
            }
        },
        PlayerState::OnWall => {
            gravity_scale.0 = 16.0;
            if velocity.linvel.y < -MAX_WALL_SLIDING_SPEED {
                velocity.linvel.y = -MAX_WALL_SLIDING_SPEED;
            }

            if buffured_jump.should_jump {
                wall_jump(&mut player, &mut velocity, &mut jump_event);
                inhibition_timer.set_duration(Duration::from_millis(250));
                inhibition_timer.reset();
            }
        },
        PlayerState::Climbing => {
            gravity_scale.0 = 0.0;
            velocity.linvel = Vec2::ZERO;
        }
    }

    player.previous_state = player.state;
}

pub fn coyote_jump(
    mut query: Query<(&Controller, &mut Player)>,
    mut modifier_query: Query<&mut Velocity, With<Player>>,
    mut timer_query: Query<&mut CoyoteTimer, With<Player>>,
    mut buffured_jump: ResMut<BufferedJump>,
    mut coyote_event: EventReader<CoyoteStart>,
    mut jump_event: EventWriter<JustJumped>,
    mut coyote_jump: ResMut<CoyoteJumpedFrom>,
    time: Res<Time>,
) {
    let (controller, mut player) = query.single_mut();
    let mut velocity = modifier_query.single_mut();
    let mut coyote_timer = timer_query.single_mut();

    if !coyote_event.is_empty() {
        for event in coyote_event.read() {
            coyote_jump.jumped_from = event.0;
            coyote_timer.reset();
        }
    } else {
        coyote_timer.tick(time.delta());
        if !coyote_timer.finished() {
            if buffured_jump.should_jump {
                if coyote_jump.jumped_from == JumpedFrom::WallOrEdge {
                    wall_jump(&mut player, &mut velocity, &mut jump_event);
                } else {
                    jump(&mut player, &controller, &mut velocity, &mut jump_event);
                }
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

fn jump(player: &mut Player, controller: &Controller, velocity: &mut Velocity, event: &mut EventWriter<JustJumped>) {
    if player.can_jump {
        debug!("Jump");
        event.send_default();
        player.jump_count += 1;
        velocity.linvel.y = JUMP_SPEED;
    }
}

fn wall_jump(player: &mut Player, velocity: &mut Velocity, event: &mut EventWriter<JustJumped>) {
    if player.can_jump {
        debug!("Wall jump");
        event.send_default();
        player.jump_count += 1;
        velocity.linvel.y = JUMP_SPEED;
        velocity.linvel.x = -player.facing_direction.x * MAX_RUNNING_SPEED;
    }
}

fn stop_vertical_velocity(velocity: &mut Velocity, force: &mut ExternalForce, max_speed: f32) {
    if velocity.linvel.y > 0.0 {
        force.force += Vec2::NEG_Y * max_speed * 200.0;
    }
}
