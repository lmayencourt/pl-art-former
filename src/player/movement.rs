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

use bevy::{prelude::*, transform, utils::info};
use bevy_rapier2d::prelude::*;

//  use crate::physics::{CollideEvent, CollideWith};
use crate::player::*;

const MAX_RUNNING_SPEED: f32 = 200.0;
const JUMP_SPEED: f32 = 300.0;
// Force to apply to reach MAX_RUNNING_SPEED in 2 secs
const RUNNING_FORCE: f32 = PLAYER_MASS/2.0 * 10.0 * MAX_RUNNING_SPEED;

pub fn player_movement(
    mut query: Query<(&Controller, &mut Player)>,
    mut physics_query: Query<(&mut ExternalForce, &mut Velocity, &mut Damping), With<Player>>,
) {
    let (controller, mut player) = query.single_mut();
    let (mut force, mut velocity, mut damping) = physics_query.single_mut();

    //  debug!("Player state {:?}", player.state);
    //  debug!("Control state {:?}", controller.direction);

    force.force = Vec2::ZERO;
    damping.linear_damping = 0.0;

    match player.state {
        PlayerState::Idle => {
            if controller.action == Action::Run {
                player.state = PlayerState::Running;
            } else if controller.action == Action::Jump {
                velocity.linvel.y = controller.direction.y * JUMP_SPEED;
                player.state = PlayerState::InAir;
            } else if controller.action == Action::None {
                // damping.linear_damping = 5.0;
            }
        },
        PlayerState::Walking => {},
        PlayerState::Running => {
            if controller.action == Action::Run {
                force.force = controller.direction * RUNNING_FORCE;
    
                if velocity.linvel.x > MAX_RUNNING_SPEED {
                    velocity.linvel.x = MAX_RUNNING_SPEED;
                }
                if velocity.linvel.x < -MAX_RUNNING_SPEED {
                    velocity.linvel.x = -MAX_RUNNING_SPEED;
                }
            } else if controller.action == Action::None {
                if velocity.linvel.x > 20.0 || velocity.linvel.x < -20.0 {
                    damping.linear_damping = 5.0;
                } else {
                    player.state = PlayerState::Idle;
                }
            }
        },
        PlayerState::InAir => {},
    }
}

//  pub fn collide_event_handler(
//      mut events: EventReader<CollideEvent>,
//      mut query: Query<&mut Player>,
//      mut next_state: ResMut<NextState<ApplicationState>>,
//  ) {
//      for event in events.read() {
//          if let CollideWith::Obstacle = event.other {
//              info!("End of Game !");
//              let mut player = query.single_mut();
//              player.attitude = PlayerAttitude::InWall;
//              next_state.set(ApplicationState::GameEnding);
//          }
//      }
//  }
