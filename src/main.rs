/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

mod player;

use player::PlayerPlugin;

fn main() {
    println!("Flappy bird made with Bevy!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        // .add_plugins(ParticleSystemPlugin)
        // // Custom plugin and systems
        // .insert_state(ApplicationState::LandingScreen)
        // .add_event::<RestartEvent>()
        // .add_systems(Startup, menu_setup)
        // .add_systems(Update, menu_control)
        // .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        // .add_plugins(PhysicsPlugin)
        .run();
}
