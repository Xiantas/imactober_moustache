mod systems;

use bevy::prelude::*;

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
        .add_systems(Startup, systems::setup)
        .run()
}
