/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Various sensor and detection for world element
use bevy::prelude::*;

use crate::DebugGizmos;
use crate::player::*;

pub fn facing_direction(mut query: Query<(&Controller, &mut Player)>) {
    let (controller, mut player) = query.single_mut();
    if controller.direction.x != 0.0 {
        if player.facing_direction.x != controller.direction.x {
            player.facing_direction.x = controller.direction.x;
        }
    }
}

pub fn ground_detection(
    mut query: Query<(&Transform, Entity, &mut Grounded), With<Player>>,
    rapier_ctx: Res<RapierContext>,
    debug: Res<DebugGizmos>,
    mut gizmos: Gizmos,
) {
    let (transform, entity, mut grounded) = query.single_mut();

    // Ray casting for ground detection
    let ray_pos = transform.translation.truncate() - Vec2::new(4.0, 0.0);
    let ray_dir = Vec2::NEG_Y;
    let max_toi = 2.1 * 16.0;
    let solid = true;
    let filter = QueryFilter::default().exclude_rigid_body(entity);

    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        grounded.0 = true;
    } else {
        grounded.0 = false;
    }

    if debug.0 {
        gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
    }

    // Ray casting for ground detection
    let ray_pos = transform.translation.truncate() + Vec2::new(4.0, 0.0);

    if !grounded.0 {
        if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            grounded.0 = true;
        } else {
            grounded.0 = false;
        }
    }

    if debug.0 {
        gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
    }
}

pub fn wall_detection(
    mut query: Query<(&Transform, Entity, &mut OnWall, &Player)>,
    rapier_ctx: Res<RapierContext>,
    debug: Res<DebugGizmos>,
    mut gizmos: Gizmos,
) {
    let (transform, entity, mut on_wall, player) = query.single_mut();

    // Ray casting for wall detection
    let ray_pos = transform.translation.truncate() + Vec2::new(0.0, 0.0);
    let ray_dir = player.facing_direction;
    let max_toi = 1.2 * 16.0;
    let solid = true;
    let filter = QueryFilter::default().exclude_rigid_body(entity);

    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        on_wall.0 = true;
    } else {
        on_wall.0 = false;
    }
    if debug.0 {
        gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
    }
}

pub fn edge_grab_detection(
    mut query: Query<(&Transform, Entity, &mut EdgeGrab, &Player)>,
    rapier_ctx: Res<RapierContext>,
    debug: Res<DebugGizmos>,
    mut gizmos: Gizmos,
) {
    let (transform, entity, mut edge_grab, player) = query.single_mut();

    // Ray casting for edge detection
    let ray_pos = transform.translation.truncate() + Vec2::new(0.0, 0.0);
    let ray_dir = player.facing_direction;
    let max_toi = 1.2 * 16.0;
    let solid = true;
    let filter = QueryFilter::default().exclude_rigid_body(entity);

    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        edge_grab.0 = true;
    } else {
        edge_grab.0 = false;
    }
    if debug.0 {
        gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
    }

    let ray_pos = transform.translation.truncate() + Vec2::new(0.0, 16.0);
    if !edge_grab.0 {
        if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
            edge_grab.0 = true;
        } else {
            edge_grab.0 = false;
        }
    }
    if debug.0 {
        gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::GREEN);
    }

    // Not an edge detection
    let ray_pos = transform.translation.truncate() + Vec2::new(0.0, 24.0);
    if let Some((_entity, _toi)) = rapier_ctx.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        // Collision above the player -> we are facing a wall and not an edge
        edge_grab.0 = false;
    }
    if debug.0 {
        gizmos.ray_2d(ray_pos, ray_dir * max_toi, Color::RED);
    }
}