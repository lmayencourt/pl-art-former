/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

use crate::player::*;

const TILE_SIZE: f32 = 8.0;
const TILE_SCALER: f32 = 4.0;

#[derive(Component)]
pub struct HoldKeySprite;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tiles.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(TILE_SIZE, TILE_SIZE), 4, 1, None, Some(Vec2::new(0.0, 8.0*5.0)));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let atlas = TextureAtlas {
        layout: texture_atlas_layout.clone(),
        index: 0,
    };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(TILE_SCALER, TILE_SCALER, 0.0),
                ..default()
            },
            visibility:Visibility::Hidden,
            ..default()
        },
        HoldKeySprite,
    ));
}

pub fn show_hold(
    player_query: Query<(&Controller, &Transform, &Player)>,
    sense_query: Query<(&Grounded, &OnWall), With<Player>>,
    mut action_event: EventReader<ActionEvent>,
    mut hold_query: Query<(&mut Visibility, &mut Transform), (With<HoldKeySprite>, Without<Player>)>
) {
    let (controller, player_transform, player) = player_query.single();
    let sensing = sense_query.single();
    let (grounded, on_wall) = sense_query.single();

    let (mut hold_visibility, mut hold_transform) = hold_query.single_mut();

    if player.state == PlayerState::Climbing {
        *hold_visibility = Visibility::Visible;

        hold_transform.translation = player_transform.translation;
        hold_transform.translation.y += 48.0;
    } else {
        *hold_visibility = Visibility::Hidden;
    }
}