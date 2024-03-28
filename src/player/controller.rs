/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Define the controls required to play the game
/// Abstract the controls from the input device, to allow playing the game
/// with a keyboard or a game-controller.
use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    None,
    Walk,
    Run,
}

#[derive(Component)]
pub struct Controller {
    pub direction: Vec2,
    pub action: Action,
}

/// Controller implementation for keyboard
pub fn keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Controller>,
) {
    let mut controller = query.single_mut();

    controller.direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        controller.direction = Vec2::NEG_X;
        controller.action = Action::Run;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        controller.direction = Vec2::X;
        controller.action = Action::Run;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        controller.direction = Vec2::Y;
    } else {
        controller.action = Action::Walk;
    }
}
