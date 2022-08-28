//!
//! Common constants and definitions
//!
use bevy::prelude::*;
pub mod prelude {
    pub use super::{
        BoardSize, Coords, ASPECT_RATIO, BACKGROUND_COLOR, BLOCK_SIZE, BOARD_SIZE,
        INITIAL_DROP_SPEED, INITIAL_POSITION, INITIAL_TRANSFORM, WORLD_HEIGHT,
    };
}

/// Struct defining the board size (in blocks)
pub struct BoardSize {
    pub width: u32,
    pub height: u32,
}

/// Struct defining the block position as x, y coordinates
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

/// Impl Coords
impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Size of the board (in blocks)
pub const BOARD_SIZE: BoardSize = BoardSize {
    width: 7,
    height: 16,
};

/// Initial block dropping speed (in seconds)
pub const INITIAL_DROP_SPEED: f32 = 1.0;

/// Initial transform for spawned blocks (Somewhere hidden)
pub const INITIAL_TRANSFORM: Transform = Transform::from_xyz(0.0, 1000.0, 1.0);

/// Initial dropping block position (in block size)
pub const INITIAL_POSITION: Coords = Coords {
    x: 4,
    y: BOARD_SIZE.height as i32 - 1,
};

/// Window aspect ratio
pub const ASPECT_RATIO: f32 = 8.0 / 12.0;

/// Pixel height of the game world
pub const WORLD_HEIGHT: f32 = 720.0;

/// Size of the game blocks
pub const BLOCK_SIZE: f32 = 64.;

/// Background color
pub const BACKGROUND_COLOR: Color = Color::Rgba {
    red: 0.55,
    blue: 0.9,
    green: 0.78,
    alpha: 1.0,
};
