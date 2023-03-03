use bevy::{prelude::*, render::camera::ScalingMode};
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
    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..Default::default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Name::new("Camera"));

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}
