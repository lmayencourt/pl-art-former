/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Manage the rendering of the different player animations from the sprite-sheet

use bevy::prelude::*;

use crate::player::*;

pub const SPRITE_RUN_IDX: (usize, usize) = (0, 7);
pub const SPRITE_WALK_IDX: (usize, usize) = (8, 15);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// True for rendering from low to high idx
/// False for high to low
// #[derive(Resource)]
// pub struct AnimationUpDown(pub bool);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut Player,
        // &mut Transform,
    )>,
    // mut animation_up: ResMut<AnimationUpDown>,
) {
    for (indices, mut timer, mut atlas, player) in &mut query {
        timer.tick(time.delta());

        // if timer.just_finished() {
        //     atlas.index = if atlas.index == indices.last {
        //         indices.first
        //     } else {
        //         atlas.index + 1
        //     }
        // }

        match player.state {
            PlayerState::Idle => {
                atlas.index = SPRITE_WALK_IDX.0;
            }
            PlayerState::Running => {
                if timer.just_finished() {
                    atlas.index = if atlas.index < SPRITE_RUN_IDX.0 {
                        SPRITE_RUN_IDX.0
                    } else if atlas.index >= SPRITE_RUN_IDX.1 {
                        SPRITE_RUN_IDX.0
                    } else {
                        atlas.index + 1
                    }
                }
            }
            PlayerState::Walking => {
                if timer.just_finished() {
                    atlas.index = if atlas.index < SPRITE_WALK_IDX.0 {
                        SPRITE_WALK_IDX.0
                    } else if atlas.index >= SPRITE_WALK_IDX.1 {
                        SPRITE_WALK_IDX.0
                    } else {
                        atlas.index + 1
                    }
                }
            }
            PlayerState::InAir => {
                atlas.index = SPRITE_RUN_IDX.0 + 2;
            }
              //     PlayerState::Dead => {
              //         atlas.index = 6;
              //         transform.rotation = Quat::from_rotation_x(std::f32::consts::PI);
              //     }
        }

        debug!("indice {}", atlas.index);
    }
}

pub fn animate_direction(
    mut query: Query<(&Velocity, &mut Sprite), With<Player>>,
) {
    let (velocity, mut sprite) = query.single_mut();

    if velocity.linvel.x > 25.0 {
        sprite.flip_x = false;
    } else if velocity.linvel.x < -25.0 {
        sprite.flip_x = true;
    }
}