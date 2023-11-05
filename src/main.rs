mod systems;
mod components;
mod resources;
mod pseudo_random;

use crate::resources::MouseInfos;

use bevy::prelude::*;
use crate::systems::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("La barbe !"),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<MouseInfos>()
        .add_systems(Startup, systems::setup)
        .add_systems(PreUpdate, (clicks_handeling, mouse_pos_updater))
        .add_systems(FixedUpdate, (fur_activation, fur_gravity_update))
        .run()
}
