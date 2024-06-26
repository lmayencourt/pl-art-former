/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

#[derive(Resource)]
struct DebugGizmos(bool);

fn main() {
    let debug_gizmos = DebugGizmos(false);
    // let debug_gizmos = DebugGizmos(true);

    App::new()
        .insert_resource(debug_gizmos)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        // Assume that the player is 2m tall
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            player::SPRITE_HEIGHT / 2.0,
        ))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ParticleSystemPlugin)
        // // Custom plugin and systems
        // .insert_state(ApplicationState::LandingScreen)
        // .add_event::<RestartEvent>()
        // .add_systems(Startup, menu_setup)
        // .add_systems(Update, menu_control)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
