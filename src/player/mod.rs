/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod controller;
pub mod movement;
pub mod sprites;
pub mod sensing;

use controller::*;
use movement::*;
use sprites::*;
use sensing::*;

pub const SPRITE_HEIGHT: f32 = 16.0;
pub const SPRITE_WIDTH: f32 = 16.0;

pub const PLAYER_MASS: f32 = 80.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    facing_direction: Vec2,
}

#[derive(Debug, Clone, Copy)]
enum PlayerState {
    Idle,
    Walking,
    Running,
    InAir,
    OnEdge,
    LeavingEdge,
    OnWall,
}

#[derive(Component)]
pub struct Grounded(bool);

#[derive(Component)]
pub struct OnWall(bool);

#[derive(Component)]
pub struct EdgeGrab(bool);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // app.add_systems(Update, restart_event_handler);
        app.add_systems(
            FixedUpdate,
            controller::keyboard_inputs, //.run_if.(in_state(ApplicationState::InGame)),
        );
        app.add_systems(FixedUpdate, sensing::facing_direction.before(player_movement));
        app.add_systems(FixedUpdate, sensing::ground_detection.before(player_movement));
        app.add_systems(FixedUpdate, sensing::wall_detection.before(player_movement));
        app.add_systems(FixedUpdate, sensing::edge_grab_detection.before(player_movement));
        app.add_systems(
            FixedUpdate,
            movement::player_movement
                .after(controller::keyboard_inputs),
                                       //.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(Update, sprites::animate_sprite.after(player_movement));
        app.add_systems(Update, sprites::animate_direction.after(player_movement));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tileset.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    commands
        .spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_xyz(0.0, -20.0, 0.0).with_scale(Vec3::splat(4.0)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player {
                state: PlayerState::Idle,
                facing_direction: Vec2::X,
                //     // jump_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
            },
            Controller {
                direction: Vec2::ZERO,
                action: Action::None,
                previous_action: Action::None,
                jump_released: true,
            },
            KeyRestTimeout(Timer::from_seconds(0.15, TimerMode::Once)),
            InhibitionTimer(Timer::from_seconds(0.3, TimerMode::Once)),
            CoyoteTimer(Timer::from_seconds(0.1, TimerMode::Once)),
            Grounded(false),
            OnWall(false),
            EdgeGrab(false),
            RigidBody::Dynamic,
        ))
        .insert(Collider::capsule(
            Vec2::new(0.0, -5.0),
            Vec2::new(0.0, 4.0),
            2.0,
        ))
        .insert(ColliderMassProperties::Mass(PLAYER_MASS))
        .insert(Ccd::enabled())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::default())
        .insert(GravityScale(16.0))
        .insert(ExternalForce::default());
}
