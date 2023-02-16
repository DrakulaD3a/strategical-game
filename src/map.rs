use bevy::prelude::*;
// use rand::Rng;

const BOARD_SIZE: isize = 16;
const TILE_SIZE: f32 = 1.0;
const SQRT_3: f32 = 1.73205080757;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_map);
    }
}

#[derive(Component)]
pub struct HexCoords {
    pub q: isize,
    pub r: isize,
}

#[derive(Component)]
pub struct Tile(TileTypes);

pub enum TileTypes {
    Normal,
}

fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let colors = [Color::RED, Color::BLUE, Color::GREEN, Color::ORANGE, Color::WHITE, Color::SILVER];

    // let mut rng = rand::thread_rng();

    let hexagon = asset_server.load("unit_hex.glb#Scene0");
    for r in -(BOARD_SIZE / 2)..(BOARD_SIZE / 2) {
        for q in -(BOARD_SIZE / 2)..(BOARD_SIZE / 2) {
            let x = TILE_SIZE * (SQRT_3 * q as f32 + SQRT_3 / 2.0 * r as f32);
            let z = TILE_SIZE * 3.0 / 2.0 * r as f32;

            commands
                .spawn(SceneBundle {
                    scene: hexagon.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, 1.0, z),
                        scale: Vec3::splat(TILE_SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(HexCoords { q, r })
                .insert(Tile(TileTypes::Normal));
        }
    }
}
