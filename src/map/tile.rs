use bevy::prelude::*;

const SQRT_3: f32 = 1.73205080757;
pub const TILE_SIZE: f32 = 8.0;
pub const WIDTH: f32 = SQRT_3 * TILE_SIZE;
pub const HEIGHT: f32 = 2.0 * TILE_SIZE;

#[derive(Component)]
pub struct Tile {
    pub q: isize,
    pub r: isize,
}

// Temporary function just for testing purposes
pub fn create_tiles(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    for z in -15..15 {
        for x in -15..15 {
            let new_x: f32 = if z % 2 == 0 {
                x as f32 * WIDTH
            } else {
                x as f32 * WIDTH + 0.5 * WIDTH
            };
            let new_z: f32 = z as f32 * (3.0 / 4.0 * HEIGHT);

            /* commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                    ..Default::default()
                },
                texture: asset_server.load("hexagon.png"),
                transform: Transform::from_xyz(new_x, new_y, 100.0),
                ..Default::default()
            }).insert(Tile { q: x, r: y }); */
            commands.spawn(SceneBundle {
                scene: asset_server.load("unit_hex.glb#Scene0"),
                transform: Transform {
                    translation: Vec3::new(new_x, 0.0, new_z),
                    scale: Vec3::splat(2.0 * TILE_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
