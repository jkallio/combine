use crate::prelude::*;
use bevy::prelude::*;

pub const BOARD_SIZE: crate::Size = crate::Size {
    width: 10,
    height: 16,
};

/// Represents the "Tetris" board
#[derive(Default)]
pub struct Board([[u8; BOARD_SIZE.width as usize]; BOARD_SIZE.height as usize]);

#[allow(dead_code)]
impl Board {
    pub fn is_solid(&self, pos: Position) -> bool {
        pos.x < 0
            || pos.x >= BOARD_SIZE.width as i32
            || pos.y < 0
            || pos.y >= BOARD_SIZE.height as i32
            || self.0[pos.y as usize][pos.x as usize] > 0
    }

    pub fn is_free(&self, pos: Position) -> bool {
        !self.is_solid(pos)
    }

    pub fn print(&self) {
        for i in 0..BOARD_SIZE.height {
            for j in 0..BOARD_SIZE.width {
                print!(
                    "{} ",
                    self.0[BOARD_SIZE.height as usize - i as usize - 1][j as usize]
                );
            }
            println!("");
        }
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::default())
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_board));
    }
}

fn on_enter(board: Res<Board>) {
    board.print();
}

fn on_exit() {}

fn update_board(mut board: ResMut<Board>, query: Query<&Position>) {
    for i in 0..BOARD_SIZE.height {
        for j in 0..BOARD_SIZE.width {
            board.0[i as usize][j as usize] = 0;
        }
    }
    for pos in query.iter() {
        board.0[pos.y as usize][pos.x as usize] = 1;
    }
}
