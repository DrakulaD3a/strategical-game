use bevy::prelude::*;

mod wfc;

use wfc::*;

const TILE_SIZE: f32 = 1.0;
const SQRT_3: f32 = 1.73205080757;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_startup_system(create_map);
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct HexCoords {
    pub q: isize,
    pub r: isize,
}

#[derive(Component)]
pub struct Tile(TileType);

#[derive(Resource)]
pub struct MapAssets {
    pub map: Handle<Image>,
    pub hexagon: Handle<Scene>,
}

fn create_map(mut commands: Commands, map_assets: Res<MapAssets>) {
    // let colors = [Color::RED, Color::BLUE, Color::GREEN, Color::ORANGE, Color::WHITE, Color::SILVER];

    // let mut rng = rand::thread_rng();

    let mut tiles = Vec::new();
    for r in -(BOARD_SIZE / 2)..(BOARD_SIZE / 2) {
        for q in -(BOARD_SIZE / 2)..(BOARD_SIZE / 2) {
            let x = TILE_SIZE * (SQRT_3 * q as f32 + SQRT_3 / 2.0 * r as f32);
            let z = TILE_SIZE * 3.0 / 2.0 * r as f32;

            let tile = commands
                .spawn(SceneBundle {
                    scene: map_assets.hexagon.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, 1.0, z),
                        scale: Vec3::splat(TILE_SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Tile"))
                .insert(HexCoords { q, r })
                .insert(Tile(TileType::Normal))
                .id();

            tiles.push(tile);
        }
    }
    commands
        .spawn(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::VISIBLE)
        .insert(ComputedVisibility::default())
        .push_children(&tiles);
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hexagon = asset_server.load("unit_hex.glb#Scene0");
    let map = asset_server.load("map.png");

    commands.insert_resource(MapAssets { map, hexagon });
}
