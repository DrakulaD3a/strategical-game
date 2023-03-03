use bevy::prelude::*;

pub struct WFCPlugin;

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_map_patterns);
    }
}

pub const BOARD_SIZE: isize = 16;

pub const SIZE_OF_MAP_PATTERN: i32 = 3;
const NUM_PATTERNS: usize = 343;
#[derive(Resource)]
pub struct MapPattern {
    pub atlas: TextureAtlas,
    pub handle: Handle<TextureAtlas>,
    pub adjacencies: [Vec<i32>; NUM_PATTERNS],
}

fn load_map_patterns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map_image = asset_server.load("map.png");
    let patterns_atlas = TextureAtlas::from_grid(
        map_image,
        Vec2::splat(SIZE_OF_MAP_PATTERN as f32),
        33,
        11,
        Some(Vec2::new(-2.0, 0.0)),
        None,
    );

    let atlas_handle = texture_atlases.add(patterns_atlas);

    commands.insert_resource(MapPattern {
        atlas: patterns_atlas,
        handle: atlas_handle,
        adjacencies: get_adjacencies(&patterns_atlas),
    });
}

fn get_adjacencies(patterns: &TextureAtlas) -> [Vec<i32>; NUM_PATTERNS] {
    let adjacencies: [Vec<i32>; NUM_PATTERNS];
    for index in 0..patterns.len() {
        let vec = Vec::new();
        for i in 0..patterns.len() {
        }
    }
    adjacencies
}
