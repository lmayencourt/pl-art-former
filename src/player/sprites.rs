/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Manage the rendering of the different player animations from the sprite-sheet
use bevy::prelude::*;
use bevy_particle_systems::*;

use crate::player::*;

pub const SPRITE_RUN_IDX: (usize, usize) = (0, 7);
pub const SPRITE_WALK_IDX: (usize, usize) = (8, 15);
pub const SPRITE_IDLE_IDX: (usize, usize) = (16, 23);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

/// Timer used to run the sprites animations
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// True for rendering from low to high idx
/// False for high to low
// #[derive(Resource)]
// pub struct AnimationUpDown(pub bool);

pub fn setup(
    mut commands: Commands,
) {
        commands
        // Add the bundle specifying the particle system itself.
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 10,
                emitter_shape: EmitterShape::Line(Line {
                    length: SPRITE_WIDTH/2.0 * SPRITE_SCALE,
                    // angle: JitteredValue::jittered(std::f32::consts::PI, -0.1..0.1),
                    angle: JitteredValue::jittered(std::f32::consts::PI/2.0, -0.3..0.3),
                }),
                spawn_rate_per_second: 50.0.into(),
                initial_speed: JitteredValue::jittered(4.0, 0.0..10.0),
                lifetime: JitteredValue::jittered(0.2, -0.1..0.2),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::rgba(0.5, 0.5, 1.0, 0.0), 1.0),
                ])),
                initial_scale: JitteredValue::jittered(3.0, -1.0..1.0),
                looping: true,
                system_duration_seconds: 0.2,
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..ParticleSystemBundle::default()
        })
        // Add the playing component so it starts playing. This can be added later as well.
        .insert(Playing);
}

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
            PlayerState::OnEdge | PlayerState::OnWall => {
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

pub fn jump_particules(
    mut commands: Commands,
    mut player_query: Query<(&Player, &Transform, &Controller)>,
    mut particule_query: Query<(Entity, &mut Transform, &mut ParticleSystem), Without<Player>>,
) {
    let (player, player_transform, controller) = player_query.single();
    let (entity, mut particule_tranform, mut particule_system) = particule_query.single_mut();

    if controller.action == Action::Jump && player.can_jump {
        particule_tranform.translation = player_transform.translation.clone();
        particule_tranform.translation.y -= SPRITE_HEIGHT/2.0 * SPRITE_SCALE;
        commands.entity(entity).insert(Playing);
    } else {
        commands.entity(entity).remove::<Playing>();
    }
}