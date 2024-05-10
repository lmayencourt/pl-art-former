/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

use crate::player::controller::*;
use crate::world::*;

#[derive(Clone, Copy)]
pub enum HoldKey {
    A,
    S,
    D,
    W,
}

// Copied from:
// https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
impl Distribution<HoldKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HoldKey {
        match rng.gen_range(0..=3) {
            0 => HoldKey::A,
            1 => HoldKey::S,
            2 => HoldKey::D,
            _ => HoldKey::W,
        }
    }
}

#[derive(Component)]
pub struct WallHold {
    pub key: HoldKey,
    pub wall_index: usize,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum HoldsVisibility {
    #[default] Hidden,
    Visible,
}

/// Inform this system to show the holds on the wall
#[derive(Event, Default)]
pub struct ShowHolds(pub HoldsVisibility);

pub fn show_hold(
    mut show_event: EventReader<ShowHolds>,
    mut wall_hold_query: Query<(&mut WallHold, &mut TextureAtlas)>
) {
    if !show_event.is_empty(){
        info!("Got new showhold event");
        for event in show_event.read() {
            match event.0 {
                HoldsVisibility::Hidden => {
                    for (mut hold, mut atlas) in wall_hold_query.iter_mut() {
                        atlas.index = hold.wall_index;
                    }
                },
                HoldsVisibility::Visible => {
                    info!("Displaying holds");
                    for (mut hold, mut atlas) in wall_hold_query.iter_mut() {
                        atlas.index = 6*4+(hold.key as usize);
                    }
                }
            }
        }
    }
}