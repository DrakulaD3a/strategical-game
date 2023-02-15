use bevy::prelude::*;

pub mod hex;
pub mod tile;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_map);
    }
}

fn create_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    tile::create_tiles(&mut commands, &asset_server);
}
