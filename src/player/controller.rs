/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Define the controls required to play the game
/// Abstract the controls from the input device, to allow playing the game
/// with a keyboard or a game-controller.
use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Action {
    #[default] None,
    Jump,
    EnterClimbingMode,
    ExitClimbingMode,
    GrabLeft,
    GrabRight,
    GrabUp,
    GrabDown,
}

const JUMP_MASK: u32 = 1;
const CLIMBING_MODE_MASK: u32 = 2;
const GRAB_LEFT_MASK: u32 = 4;
const GRAB_RIGHT_MASK: u32 = 8;
const GRAB_UP_MASK: u32 = 16;
const GRAB_DOWN_MASK: u32 = 32;

/// Inform other system of an action to perform
#[derive(Event, Default)]
pub struct ActionEvent(pub Action);

#[derive(Component, Debug)]
pub struct Controller {
    pub direction: Vec2,
    pub action: Action,
    pub previous_action: Action,
    pub jump_released: bool,
    pub action_vector: u32,
}

/// Controller implementation for keyboard
pub fn keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Controller>,
    mut event: EventWriter<ActionEvent>,
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
        if controller.jump_released {
            event.send(ActionEvent(Action::Jump));
            controller.jump_released = false;
        }
    }

    // Allow jumping when key is released.
    if controller.previous_action == Action::Jump && !keyboard_input.pressed(KeyCode::Space) {
        controller.jump_released = true;
    }

    // Climbing inputs
    let climb_mode = (controller.action_vector & CLIMBING_MODE_MASK) != 0;
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        if !climb_mode {
            event.send(ActionEvent(Action::EnterClimbingMode));
            controller.action_vector |= CLIMBING_MODE_MASK;
        }
    } else if !keyboard_input.pressed(KeyCode::ShiftLeft) && !keyboard_input.pressed(KeyCode::ShiftRight){
        if climb_mode {
            event.send(ActionEvent(Action::ExitClimbingMode));
            controller.action_vector &= !CLIMBING_MODE_MASK;
        }
    }

    if climb_mode {
        if handle_key_input(&mut controller, GRAB_LEFT_MASK, keyboard_input.pressed(KeyCode::KeyA)) {
            event.send(ActionEvent(Action::GrabLeft));
        }
        if handle_key_input(&mut controller, GRAB_RIGHT_MASK, keyboard_input.pressed(KeyCode::KeyD)) {
            event.send(ActionEvent(Action::GrabRight));
        }
        if handle_key_input(&mut controller, GRAB_UP_MASK, keyboard_input.pressed(KeyCode::KeyW)) {
            event.send(ActionEvent(Action::GrabUp));
        }
        if handle_key_input(&mut controller, GRAB_DOWN_MASK, keyboard_input.pressed(KeyCode::KeyS)) {
            event.send(ActionEvent(Action::GrabDown));
        }
    }

    controller.previous_action = controller.action;
}

fn handle_key_input(controller: &mut Controller, action_mask: u32, key_pressed: bool) -> bool {
    let action = controller.action_vector & action_mask != 0;
    if key_pressed {
        if !action {
            controller.action_vector |= action_mask;
            return true
        }
    } else {
        controller.action_vector &= !action_mask;
    }

    false
}
