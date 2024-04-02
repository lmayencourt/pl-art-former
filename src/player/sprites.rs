/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Manage the rendering of the different player animations from the sprite-sheet
use bevy::prelude::*;

use crate::player::*;

pub const SPRITE_RUN_IDX: (usize, usize) = (0, 7);
pub const SPRITE_WALK_IDX: (usize, usize) = (8, 15);
pub const SPRITE_IDLE_IDX: (usize, usize) = (16, 23);

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
    )>,
) {
    for (indices, mut timer, mut atlas, player) in &mut query {
        timer.tick(time.delta());

        match player.state {
            PlayerState::Idle => {
                if timer.just_finished() {
                    atlas.index = if atlas.index < SPRITE_IDLE_IDX.0 {
                        SPRITE_IDLE_IDX.0
                    } else if atlas.index >= SPRITE_IDLE_IDX.1 {
                        SPRITE_IDLE_IDX.0
                    } else {
                        atlas.index + 1
                    }
                }
            },
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
            },
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
            },
            PlayerState::InAir => {
                atlas.index = SPRITE_RUN_IDX.0 + 2;
            },
            PlayerState::OnEdge | PlayerState::LeavingEdge | PlayerState::OnWall => {
                atlas.index = SPRITE_WALK_IDX.0 + 5;
            }
        }

        debug!("indice {}", atlas.index);
    }
}

pub fn animate_direction(mut query: Query<(&Player, &mut Sprite)>) {
    let (player, mut sprite) = query.single_mut();

    if player.facing_direction == Vec2::X {
        sprite.flip_x = false;
    } else {
        sprite.flip_x = true;
    }
}
