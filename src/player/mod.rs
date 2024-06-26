/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod camera;
pub mod controller;
pub mod movement;
pub mod sprites;
pub mod sensing;

use camera::*;
use controller::*;
use movement::*;
use sprites::*;
use sensing::*;

pub const SPRITE_HEIGHT: f32 = 16.0;
pub const SPRITE_WIDTH: f32 = 16.0;
pub const SPRITE_SCALE: f32 = 4.0;

pub const PLAYER_MASS: f32 = 80.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    state: PlayerState,
    previous_state: PlayerState,
    facing_direction: Vec2,
    jump_count: u32,
    can_jump: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerState {
    Idle,
    Walking,
    Running,
    InAir,
    OnEdge,
    OnWall,
    Climbing,
}

#[derive(Component)]
pub struct Grounded(bool);

#[derive(Component)]
pub struct OnWall(bool);

#[derive(Component)]
pub struct EdgeGrab(bool);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JustJumped>();
        app.add_event::<CoyoteStart>();
        app.add_event::<ActionEvent>();
        app.insert_resource(CoyoteJumpedFrom{jumped_from: JumpedFrom::Ground});
        app.insert_resource(BufferedJump {
            should_jump: false,
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        });
        app.add_systems(Startup, setup);
        app.add_systems(Startup, sprites::setup);
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
        app.add_systems(FixedUpdate, movement::coyote_jump.after(controller::keyboard_inputs));
        app.add_systems(Update, sprites::animate_sprite.after(player_movement));
        app.add_systems(Update, sprites::animate_direction.after(player_movement));
        app.add_systems(Update, sprites::jump_particules);
        app.add_systems(Update, camera::follow_player);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tileset.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT), 8, 5, None, None);
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
                transform: Transform::from_xyz(0.0, -20.0, 0.0).with_scale(Vec3::splat(SPRITE_SCALE)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player {
                state: PlayerState::Idle,
                previous_state: PlayerState::Idle,
                facing_direction: Vec2::X,
                jump_count: 0,
                can_jump: true,
                //     // jump_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
            },
            Controller {
                direction: Vec2::ZERO,
                action: Action::None,
                previous_action: Action::None,
                jump_released: true,
                action_vector: 0,
            },
            InhibitionTimer(Timer::from_seconds(0.25, TimerMode::Once)),
            CoyoteTimer(Timer::from_seconds(0.1, TimerMode::Once)),
            JumpParticulesTimer(Timer::from_seconds(0.1, TimerMode::Once)),
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
