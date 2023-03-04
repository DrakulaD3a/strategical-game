use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod map;

use map::*;

fn main() {
    let height = 720.0;
    let width = 16.0 / 9.0 * height;

    App::new()
        // Plugin set for spawning window
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width,
                        height,
                        title: "Strategy".to_string(),
                        present_mode: bevy::window::PresentMode::Fifo,
                        scale_factor_override: Some(1.0),
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // Plugin for debugging
        .add_plugin(WorldInspectorPlugin)
        // Custom plugins
        .add_plugin(MapPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
