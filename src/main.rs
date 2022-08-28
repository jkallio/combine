//!
//! Simple Tetris style math puzzle game which I made for Bevy Jam #2
//! License: MIT
//! Year: 2022
//! Author: Jussi Kallio
//!
//! Music by Eric Matyas
//! www.soundimage.org
//!
use audio::AudioPlugin;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use constants::prelude::*;
use game_over::GameOverPlugin;
use in_game::InGamePlugin;
use menu::MenuPlugin;
mod audio;
mod board;
mod constants;
mod game_over;
mod in_game;
mod menu;

/// Resource for holding the window size
pub struct WindowSize(Vec2);

pub mod prelude {
    pub use super::{GameState, HighScore, Score, WindowSize};
}

/// Event which launches the Main Menu
pub struct LaunchMenuEvent;

/// Defines the main state machine states
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Init,
    Menu,
    InGame,
    GameOver,
}

/// Resource for storing the score
pub struct Score(i32);
pub struct HighScore(i32);

fn main() {
    let win_size = Vec2::new(WORLD_HEIGHT * ASPECT_RATIO, WORLD_HEIGHT);

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Combine".into(),
            width: win_size.x,
            height: win_size.y,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowSize(win_size))
        .insert_resource(Score(0))
        .insert_resource(HighScore(0))
        .add_event::<LaunchMenuEvent>()
        .add_state(GameState::Init)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(GameOverPlugin)
        .add_startup_system(game_setup)
        .add_system_set(SystemSet::on_update(GameState::Init).with_system(launch_menu))
        .run();
}

pub fn game_setup(mut commands: Commands, mut launch_event: EventWriter<LaunchMenuEvent>) {
    let mut camera = Camera2dBundle::default();

    // No re-scaling on windows resize
    camera.projection.scaling_mode = ScalingMode::None;
    camera.projection.scale = WORLD_HEIGHT / 1.5;

    // Fix the aspect ratio
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.left = -1.0 * ASPECT_RATIO;
    camera.projection.right = 1.0 * ASPECT_RATIO;

    // Spawn the camera
    commands.spawn_bundle(camera);

    launch_event.send(LaunchMenuEvent);
}

fn launch_menu(
    mut game_state: ResMut<State<GameState>>,
    mut launch_event: EventReader<LaunchMenuEvent>,
) {
    for _ in launch_event.iter() {
        game_state
            .set(GameState::Menu)
            .expect("Failed to set GameState::Menu");
    }
}
