/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod levels;

use levels::*;

pub struct WorldPlugin;

const TILE_SIZE: f32 = 8.0;
const TILE_SCALER: f32 = 4.0;
pub const TILE_SCALED: f32 = TILE_SIZE * TILE_SCALER;

/// World size definition
const WORLD_HEIGHT: f32 = 600.0;
const WORLD_TOP: f32 = WORLD_HEIGHT / 2.0;
const WORLD_BOTTOM: f32 = -WORLD_TOP;
// Use the golden ration here for the world size
const WORLD_WIDTH: f32 = WORLD_HEIGHT * 1.618;
const WORLD_RIGHT: f32 = WORLD_WIDTH / 2.0;
const WORLD_LEFT: f32 = -WORLD_RIGHT;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
        // app.add_systems(Update, debug_grid);
    }
}

fn debug_grid(mut gizmos: Gizmos) {
    for x in 0..512 {
        if x % TILE_SCALED as u32 == 0 {
            gizmos.line_2d(
                Vec2::new(
                    WORLD_LEFT - TILE_SCALED / 2.0,
                    WORLD_TOP + TILE_SCALED / 2.0 - (x as f32),
                ),
                Vec2::new(
                    WORLD_RIGHT - TILE_SCALED / 2.0,
                    WORLD_TOP + TILE_SCALED / 2.0 - (x as f32),
                ),
                Color::GRAY,
            );
        }
    }

    for y in 0..1024 {
        if y % TILE_SCALED as u32 == 0 {
            gizmos.line_2d(
                Vec2::new(
                    WORLD_LEFT - TILE_SCALED / 2.0 + (y as f32),
                    WORLD_TOP + TILE_SCALED / 2.0,
                ),
                Vec2::new(
                    WORLD_LEFT - TILE_SCALED / 2.0 + (y as f32),
                    WORLD_BOTTOM - TILE_SCALED / 2.0,
                ),
                Color::GRAY,
            );
        }
    }
}

fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 2d Camera
    commands.spawn(Camera2dBundle::default());

    // Background
    let texture = asset_server.load("background_mountains.png");
    commands.spawn(SpriteBundle{
        texture: texture,
        transform: Transform {
            translation: Vec3::new(0.0, 200.0, -10.0),
            scale: Vec3::new(TILE_SCALER, TILE_SCALER, 0.0),
            ..default()
        },
        ..default()
    });

    // Tile-set
    let texture = asset_server.load("tiles.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(TILE_SIZE, TILE_SIZE), 4, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for (y, line) in LEVEL_TRAINING.lines().enumerate() {
        println!("line is {:?}", line);
        for (x, char) in line.chars().enumerate() {
            let translation = Vec3::new(
                WORLD_LEFT + x as f32 * TILE_SCALED,
                WORLD_TOP - y as f32 * TILE_SCALED,
                0.0,
            );
            let scale = Vec3::new(TILE_SCALER, TILE_SCALER, 0.0);
            let style = 3;
            let idx = if char == 'B' {
                Some(0 + style*4)
            } else if char == 'b' {
                Some(0 + (style+1)*4)
            } else if char == 'R' {
                Some(1 + style*4)
            } else if char == 'r' {
                Some(1 + (style+1)*4)
            } else if char == 'G' {
                Some(2 + style*4)
            } else if char == 'g' {
                Some(2 + (style+1)*4)
            } else if char == 'D' {
                Some(3 + style*4)
            } else if char == 'd' {
                Some(3 + (style+1)*4)
            } else {
                None
            };
            if let Some(idx) = idx {
                let atlas = TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: idx,
                };
                spawn_wall(&mut commands, translation, scale, texture.clone(), atlas);
            }
        }
    }
}

fn spawn_wall(commands: &mut Commands, translation: Vec3, scale: Vec3, texture: Handle<Image>, atlas: TextureAtlas) {
    commands
        .spawn((SpriteSheetBundle {
            texture,
            atlas,
            transform: Transform {
                translation: translation,
                scale: scale,
                ..default()
            },
            ..default()
        },))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/2.0));
}
