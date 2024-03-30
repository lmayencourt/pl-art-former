/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use std::fs::File;
use std::io::{BufReader, BufRead};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

const TILE_SIZE: f32 = 16.0 * 2.0;

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

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
        app.add_systems(Update, debug_grid);
    }
}

fn debug_grid(mut gizmos:Gizmos) {
    for x in 0..512 {
        if x%TILE_SIZE as u32 == 0 {
            gizmos.line_2d(Vec2::new(WORLD_LEFT - TILE_SIZE/2.0, WORLD_TOP + TILE_SIZE/2.0 - (x as f32)), Vec2::new(WORLD_RIGHT - TILE_SIZE/2.0, WORLD_TOP + TILE_SIZE/2.0 - (x as f32)), Color::GRAY);
        }
    }

    for y in 0..1024 {
        if y%TILE_SIZE as u32 == 0 {
            gizmos.line_2d(Vec2::new(WORLD_LEFT - TILE_SIZE/2.0 + (y as f32), WORLD_TOP + TILE_SIZE/2.0), Vec2::new(WORLD_LEFT - TILE_SIZE/2.0 + (y as f32), WORLD_BOTTOM - TILE_SIZE/2.0), Color::GRAY);
        }
    }
}

fn setup_world(mut commands: Commands) {
    // 2d Camera
    commands.spawn(Camera2dBundle::default());

    let file = File::open("assets/map.txt").expect("No world map found in assets/ folder");
    for (y, line) in BufReader::new(file).lines().enumerate() {
        println!("line is {:?}", line);
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let translation = Vec3::new(WORLD_LEFT + x as f32 *TILE_SIZE, WORLD_TOP - y as f32 *TILE_SIZE, 0.0);
                let scale = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
                if char == '#'{
                    spawn_wall(&mut commands, translation, scale);
                }
            }
        }
    }
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