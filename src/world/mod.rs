/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

/// World size definition
const WORLD_HEIGHT: f32 = 600.0;
const WORLD_TOP: f32 = WORLD_HEIGHT / 2.0;
const WORLD_BOTTOM: f32 = -WORLD_TOP;
// Use the golden ration here for the world size
const WORLD_WIDTH: f32 = WORLD_HEIGHT * 1.618;
const WORLD_RIGHT: f32 = WORLD_WIDTH / 2.0;
const WORLD_LEFT: f32 = -WORLD_RIGHT;
const WORLD_HEIGHT_MIDDLE: f32 = 0.0; //WORLD_BOTTOM + WORLD_HEIGHT/2.0;
const WORLD_WIDTH_MIDDLE: f32 = 0.0;

const BORDER_WIDTH: f32 = 20.0;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(mut commands: Commands) {
    // 2d Camera
    commands.spawn(Camera2dBundle::default());

    // world borders
    spawn_wall(&mut commands, Vec3::new(WORLD_LEFT, WORLD_HEIGHT_MIDDLE, 0.0), Vec3::new(BORDER_WIDTH, WORLD_HEIGHT, 0.0));
    spawn_wall(&mut commands, Vec3::new(WORLD_RIGHT, WORLD_HEIGHT_MIDDLE, 0.0), Vec3::new(BORDER_WIDTH, WORLD_HEIGHT, 0.0));
    spawn_wall(&mut commands, Vec3::new(WORLD_WIDTH_MIDDLE, WORLD_TOP, 0.0), Vec3::new(WORLD_WIDTH, BORDER_WIDTH, 0.0));
    spawn_wall(&mut commands, Vec3::new(WORLD_WIDTH_MIDDLE, WORLD_BOTTOM, 0.0), Vec3::new(WORLD_WIDTH, BORDER_WIDTH, 0.0));

    // Platform
    spawn_wall(&mut commands, Vec3::new(WORLD_LEFT + WORLD_WIDTH/4.0, WORLD_BOTTOM + 16.0*4.0*2.0 - BORDER_WIDTH, 0.0), Vec3::new(150.0, BORDER_WIDTH, 0.0));
    spawn_wall(&mut commands, Vec3::new(WORLD_WIDTH_MIDDLE, WORLD_BOTTOM + 16.0*4.0*4.0 - BORDER_WIDTH, 0.0), Vec3::new(150.0, BORDER_WIDTH, 0.0));
    spawn_wall(&mut commands, Vec3::new(WORLD_RIGHT - WORLD_WIDTH/4.0, WORLD_BOTTOM + WORLD_HEIGHT*3.0/4.0, 0.0), Vec3::new(150.0, BORDER_WIDTH, 0.0));
}

fn spawn_wall(commands: &mut Commands, translation: Vec3, scale: Vec3) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: translation,
                scale: scale,
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
    ))
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5));
}