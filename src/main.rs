//! Bevy Game Jam #2 Entry
//!
//! Idea is to create a puzzle game where player can pick any two objects to create a tool
//! that can overcome an obstacle. E.g. combine stick with rock to create a hammer
//!
//! Okay... Another idea is to create a tetris-like game where number blocks drop from the sky
//! and the goal is to get them disappear by combining them using somekind of math
//!
//! TODO:
//!     - Create a system to pick objects
//!     - Create a two slot inventory
//!     - Draw the game art
//!
//! Combine Ideas:
//!     - Stick + Rock = Hammer
//!     - Flint + Steel = Tinderbox
//!     - Stick + Steel = Pickaxe

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use in_game::InGamePlugin;
use menu::MenuPlugin;
mod collider;
mod in_game;
mod menu;

/// Window aspect ratio
const ASPECT_RATIO: f32 = 8.0 / 10.0;

/// Pixel height of the game world
const WORLD_HEIGHT: f32 = 240.0;

/// Window scale factor
const WINDOW_SCALE: f32 = 3.0;

/// Size of the game blocks
pub const BLOCK_SIZE: f32 = 64.;

/// Resource for holding the window size
pub struct WindowSize(Vec2);

pub mod prelude {
    pub use super::{GameCamera, GameState, BLOCK_SIZE};
}

pub struct LaunchMenu;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Init,
    Menu,
    InGame,
}

#[derive(Component)]
pub struct GameCamera;

fn main() {
    let win_size = Vec2::new(WORLD_HEIGHT * ASPECT_RATIO, WORLD_HEIGHT);

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Combine".into(),
            width: win_size.x * WINDOW_SCALE,
            height: win_size.y * WINDOW_SCALE,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLUE))
        .insert_resource(WindowSize(win_size))
        .add_event::<LaunchMenu>()
        .add_state(GameState::Init)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(InGamePlugin)
        .add_startup_system(game_setup)
        .add_system_set(SystemSet::on_update(GameState::Init).with_system(launch_menu))
        .run();
}

pub fn game_setup(mut commands: Commands, mut launch_event: EventWriter<LaunchMenu>) {
    let mut camera = Camera2dBundle::default();

    // No re-scaling on windows resize
    camera.projection.scaling_mode = ScalingMode::None;
    camera.projection.scale = 600.0;

    // Fix the aspect ratio
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.left = -1.0 * ASPECT_RATIO;
    camera.projection.right = 1.0 * ASPECT_RATIO;

    // Spawn the camera
    commands.spawn_bundle(camera).insert(GameCamera);

    launch_event.send(LaunchMenu);
}

fn launch_menu(
    mut game_state: ResMut<State<GameState>>,
    mut launch_event: EventReader<LaunchMenu>,
) {
    for _ in launch_event.iter() {
        game_state
            .set(GameState::Menu)
            .expect("Failed to set GameState::Menu");
    }
}
