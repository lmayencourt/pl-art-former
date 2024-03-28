/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(ParticleSystemPlugin)
        // // Custom plugin and systems
        // .insert_state(ApplicationState::LandingScreen)
        // .add_event::<RestartEvent>()
        // .add_systems(Startup, menu_setup)
        // .add_systems(Update, menu_control)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        // .add_plugins(PhysicsPlugin)
        .run();
}
