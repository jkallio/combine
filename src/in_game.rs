use crate::prelude::*;
use bevy::prelude::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(back_to_menu_on_esc),
            );
    }
}

fn on_enter() {
    println!("Enter GameState::InGame");
}

fn on_exit() {
    println!("Exit GameState::InGame");
}

pub fn back_to_menu_on_esc(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) && *game_state.current() != GameState::Menu {
        game_state
            .set(GameState::Menu)
            .expect("Failed to change GameState::Menu");
        input.reset(KeyCode::Escape);
    }
}
