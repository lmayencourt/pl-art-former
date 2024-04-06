/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Define the controls required to play the game
/// Abstract the controls from the input device, to allow playing the game
/// with a keyboard or a game-controller.
use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action {
    None,
    Jump,
}

#[derive(Component, Debug)]
pub struct Controller {
    pub direction: Vec2,
    pub action: Action,
    pub previous_action: Action,
    pub jump_released: bool,
}

/// Controller implementation for keyboard
pub fn keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Controller>,
    time: Res<Time>,
) {
    let mut controller = query.single_mut();

    controller.action = Action::None;
    controller.direction = Vec2::ZERO;

    // Directional inputs
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        controller.direction = Vec2::NEG_X;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        controller.direction = Vec2::X;
    }

    // Jump inputs
    if keyboard_input.pressed(KeyCode::Space) {
        controller.direction += Vec2::Y;
        controller.action = Action::Jump;
        controller.jump_released = false;
    }

    // Allow jumping when key is released.
    if controller.previous_action == Action::Jump && !keyboard_input.pressed(KeyCode::Space) {
        controller.jump_released = true;
    }

    controller.previous_action = controller.action;
}
