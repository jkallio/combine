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
use how_to_play::HowToPlayPlugin;
use in_game::{get_translation, BlockPosition, InGamePlugin};
use menu::MenuPlugin;
mod audio;
mod board;
mod constants;
mod game_over;
mod how_to_play;
mod in_game;
mod menu;

/// Resource for holding the window size
pub struct WindowSize(Vec2);

pub mod prelude {
    pub use super::{
        EdgeBlock, GameObject, GameState, HighScore, MenuNode, MyAssets, Score, WindowSize,
    };
}

/// Tag for MenuItems
#[derive(Component)]
pub struct MenuNode;

/// All game objects are tagged with this `Component` for easier clean-up
#[derive(Component)]
pub struct GameObject;

/// Identifies edge blocks
#[derive(Component)]
pub struct EdgeBlock;

/// Event which launches the Main Menu
pub struct LaunchMenuEvent;

/// Defines the main state machine states
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Init,
    Menu,
    InGame,
    GameOver,
    HowToPlay,
}

/// Block image texture and text style is preloaded in this resource
pub struct MyAssets {
    text_style: TextStyle,
    block_texture: Handle<Image>,
    edge_texture: Handle<Image>,
    how_to_play_1_texture: Handle<Image>,
    how_to_play_2_texture: Handle<Image>,
}

/// This `Event` is sent when edge blocks need to be spawned
pub struct SpawnEdgeBlockEvent {
    position: Coords,
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
        .add_plugin(HowToPlayPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(GameOverPlugin)
        .add_startup_system(game_setup)
        .add_system_set(SystemSet::on_update(GameState::Init).with_system(launch_menu))
        .add_system_set(SystemSet::on_update(GameState::Init).with_system(spawn_edge_block))
        .add_event::<SpawnEdgeBlockEvent>()
        .run();
}

pub fn game_setup(
    mut commands: Commands,
    mut launch_event: EventWriter<LaunchMenuEvent>,
    mut block_event: EventWriter<SpawnEdgeBlockEvent>,
    asset_server: Res<AssetServer>,
) {
    let mut camera = Camera2dBundle::default();

    // Load the assets
    let font = asset_server.load("fonts/04b_30.ttf");
    // Insert Block Text Style as resoruce
    commands.insert_resource(MyAssets {
        text_style: TextStyle {
            font: font.clone(),
            font_size: 24.0,
            color: Color::BLACK,
        },
        block_texture: asset_server.load("pixel-block.png"),
        edge_texture: asset_server.load("edge-block.png"),
        how_to_play_1_texture: asset_server.load("how-to-play1.png"),
        how_to_play_2_texture: asset_server.load("how-to-play2.png"),
    });

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

    // Initialize the game area (edges)
    for y in -1..=BOARD_SIZE.height as i32 {
        for x in -1..=BOARD_SIZE.width as i32 {
            if y < 0 || y >= BOARD_SIZE.height as i32 || x < 0 || x >= BOARD_SIZE.width as i32 {
                let coords = Coords::new(x, y);
                block_event.send(SpawnEdgeBlockEvent {
                    position: coords.clone(),
                });
            }
        }
    }

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

/// System for spawning edge blocks
fn spawn_edge_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEdgeBlockEvent>,
    win_size: Res<WindowSize>,
    my_assets: Res<MyAssets>,
) {
    for ev in event_reader.iter() {
        let _ = commands
            .spawn_bundle(SpriteBundle {
                texture: my_assets.edge_texture.clone(),
                transform: Transform::from_translation(get_translation(&win_size.0, &ev.position)),
                ..default()
            })
            .insert(EdgeBlock)
            .insert(BlockPosition(ev.position))
            .id();
    }
}
