/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Various sensor and detection for world element
use bevy::prelude::*;

use crate::player::*;

pub fn facing_direction(mut query: Query<(&Velocity, &mut Sprite, &mut Player)>) {
    let (velocity, mut sprite, mut player) = query.single_mut();

    if velocity.linvel.x > 25.0 {
        player.facing_direction = Vec2::X;
    } else if velocity.linvel.x < -25.0 {
        player.facing_direction = Vec2::NEG_X;
    }
}

pub fn ground_detection(
    mut query: Query<(&Transform, Entity, &mut Grounded), With<Player>>,
    rapier_ctx: Res<RapierContext>,
    mut gizmos: Gizmos,
) {
    let (transform, entity, mut grounded) = query.single_mut();

    // Ray casting for ground detection
    let ray_pos = transform.translation.truncate() - Vec2::new(8.0, 0.0);
    let ray_dir = Vec2::NEG_Y;
    let max_toi = 2.5 * 16.0;
    let solid = true;
    let filter = QueryFilter::default().exclude_rigid_body(entity);

    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        grounded.0 = true;
    } else {
        grounded.0 = false;
    }

    // gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);

    // Ray casting for ground detection
    let ray_pos = transform.translation.truncate() + Vec2::new(8.0, 0.0);

    if !grounded.0 {
        if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            grounded.0 = true;
        } else {
            grounded.0 = false;
        }
    }

    // gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
}

pub fn edge_grab_detection(
    mut query: Query<(&Transform, Entity, &mut EdgeGrab, &Player)>,
    rapier_ctx: Res<RapierContext>,
    mut gizmos: Gizmos,
) {
    let (transform, entity, mut edge_grab, player) = query.single_mut();

    // Ray casting for ground detection
    let ray_pos = transform.translation.truncate() + Vec2::new(0.0, 0.0);
    let ray_dir = player.facing_direction;
    let max_toi = 2.0 * 16.0;
    let solid = true;
    let filter = QueryFilter::default().exclude_rigid_body(entity);

    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        edge_grab.0 = true;
    } else {
        edge_grab.0 = false;
    }

    gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);

    let ray_pos = transform.translation.truncate() + Vec2::new(0.0, 16.0);
  
    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        edge_grab.0 = true;
    } else {
        edge_grab.0 = false;
    }

    gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
}