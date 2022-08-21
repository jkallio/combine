use crate::prelude::*;
use bevy::prelude::*;
use bevy::window::close_on_esc;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(on_exit))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(start_game_on_enter))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(close_on_esc));
    }
}

fn on_enter() {
    println!("Enter GameState::Menu");
}

fn on_exit() {
    println!("Exit GameState::Menu");
}

fn start_game_on_enter(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Return) {
        game_state
            .set(GameState::InGame)
            .expect("Failed to change GameState:InGame");
        input.reset(KeyCode::Return);
    }
}
