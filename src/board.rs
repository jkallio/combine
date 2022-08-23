use crate::{in_game::Number, in_game::SolidBlock, prelude::*};
use bevy::prelude::*;

pub const BOARD_SIZE: crate::Size = crate::Size {
    width: 7,
    height: 16,
};

/// Represents the "Tetris" board
#[derive(Default)]
pub struct Board([[Option<Entity>; BOARD_SIZE.width as usize]; BOARD_SIZE.height as usize]);

#[allow(dead_code)]
impl Board {
    pub fn is_occupied(&self, pos: BlockPosition) -> bool {
        pos.x < 0
            || pos.x >= BOARD_SIZE.width as i32
            || pos.y < 0
            || pos.y >= BOARD_SIZE.height as i32
            || self.0[pos.y as usize][pos.x as usize].is_some()
    }

    pub fn is_free(&self, pos: BlockPosition) -> bool {
        !self.is_occupied(pos)
    }

    pub fn print(&self) {
        for i in 0..BOARD_SIZE.height {
            for j in 0..BOARD_SIZE.width {
                if self.0[BOARD_SIZE.height as usize - i as usize - 1][j as usize].is_some() {
                    print!("X ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }
        println!("-------------");
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

fn on_exit(mut board: ResMut<Board>) {
    for i in 0..BOARD_SIZE.height {
        for j in 0..BOARD_SIZE.width {
            board.0[i as usize][j as usize] = None;
        }
    }
}

fn update_board(
    mut board: ResMut<Board>,
    query: Query<(Entity, &BlockPosition, &Number), Added<SolidBlock>>,
) {
    for (entity, pos, _) in query.iter() {
        board.0[pos.y as usize][pos.x as usize] = Some(entity);
        board.print();
    }
}
