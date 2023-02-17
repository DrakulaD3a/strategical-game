use bevy::prelude::*;
use crate::TileType;
use rand::Rng;

use super::BOARD_SIZE;

// To modify the map, change this impl
impl TileType {
    // This has to be equal to the number of different tiles
    const NUM_PATTERNS: usize = 1;

    // Add a branch to the match (switch) for each new tile
    fn from_usize(n: usize) -> Option<Self> {
        use TileType::*;
        match n {
            0 => Some(Normal),
            _ => None,
        }
    }
}

fn wfc(mut commands: Commands) {
    let wave = [[true; TileType::NUM_PATTERNS]; (BOARD_SIZE * BOARD_SIZE) as usize];
    let entropies = [TileType::NUM_PATTERNS; (BOARD_SIZE * BOARD_SIZE) as usize];
    let map: [TileType; (BOARD_SIZE * BOARD_SIZE) as usize];

    // Find the lowest entropy and call collapse
    // Update entropies
    // Repeat until all entropies == 1
}

fn update_entropies(
    wave: [[bool; TileType::NUM_PATTERNS]; (BOARD_SIZE * BOARD_SIZE) as usize],
    entropies: &mut [usize; (BOARD_SIZE * BOARD_SIZE) as usize],
) {
    for (index, entropy) in entropies.iter_mut().enumerate() {
        *entropy = wave[index]
            .iter()
            .filter(|&&wave| wave)
            .count();
    }
}

fn collapse(tile: &mut [bool; TileType::NUM_PATTERNS]) -> Option<TileType> {
    let true_indices: Vec<usize> = tile
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| if value { Some(index) } else { None })
        .collect();

    let mut rng = rand::thread_rng();

    let tile_type = rng.gen_range(0..true_indices.len());

    TileType::from_usize(tile_type)
}
