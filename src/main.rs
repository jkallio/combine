//! Bevy Game Jam #2 Entry
//!
//! Idea is to create a puzzle game where player can pick any two objects to create a tool
//! that can overcome an obstacle. E.g. combine stick with rock to create a hammer
//!
//! TODO:
//!     - Create a Main Menu and In game States
//!     - Create a player and move it with arrow keys
//!     - Create a system to pick objects
//!     - Create a two slot inventory
//!     - Draw the game art
//!
//! Combine Ideas:
//!     - Stick + Rock = Hammer
//!     - Flint + Steel = Tinderbox
//!     - Stick + Steel = Pickaxe

use bevy::prelude::*;
use in_game::InGamePlugin;
use menu::MenuPlugin;
mod in_game;
mod menu;

pub mod prelude {
    pub use super::{GameCamera, GameState};
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
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLUE))
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
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameCamera);

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
