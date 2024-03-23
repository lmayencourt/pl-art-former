/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Define the controls required to play the game
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

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        // controller.direction.y = 1.0;
        controller.action = Action::Run;
    } else {
        controller.action = Action::Walk;
    }
}
