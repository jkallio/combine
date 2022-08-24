use crate::in_game::{
    BlockColor, LastDroppedBlock, Number, Operation, PerformCalculationEvent, SolidBlock,
};
use crate::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub const BOARD_SIZE: crate::Size = crate::Size {
    width: 7,
    height: 16,
};

type BlockInfo = Option<(Entity, BlockColor)>;

pub struct BlockMap(HashMap<BlockPosition, BlockInfo>);
impl BlockMap {
    pub fn new_empty() -> BlockMap {
        let block_map = HashMap::<BlockPosition, BlockInfo>::new();
        BlockMap(block_map)
    }

    pub fn is_none(&self, pos: &BlockPosition) -> bool {
        if (0..BOARD_SIZE.width).contains(&(pos.x as u32))
            && (0..BOARD_SIZE.height).contains(&(pos.y as u32))
        {
            if let Some(value) = self.0.get(pos) {
                return value.is_none();
            } else {
                return true;
            }
        }
        false
    }

    pub fn get_block(&self, pos: &BlockPosition) -> BlockInfo {
        if let Some(value) = self.0.get(pos) {
            return value.clone();
        }
        None
    }

    pub fn set_block(&mut self, pos: &BlockPosition, value: BlockInfo) {
        if value.is_some() {
            self.0.insert(*pos, value);
        } else if self.get_block(pos).is_some() {
            self.0.remove(pos);
        }
    }

    pub fn print_debug(&self) {
        for y in (-1..=BOARD_SIZE.height as i32).rev() {
            for x in -1..=BOARD_SIZE.width as i32 {
                if self.is_none(&BlockPosition::new(x, y)) {
                    print!(". ");
                } else {
                    print!("# ");
                }
            }
            println!("");
        }
        println!("---------------");
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_board))
            .insert_resource(BlockMap::new_empty());
    }
}

fn on_enter(mut block_map: ResMut<BlockMap>) {
    block_map.0.clear();
    block_map.print_debug();
}

fn on_exit(mut block_map: ResMut<BlockMap>) {
    block_map.0.clear();
}

/// Finds same colored neighbors by calling itself recursively for each found neighbor
fn find_same_color_neighbors(
    block_map: &BlockMap,
    pos: &BlockPosition,
    color: BlockColor,
    mut neighbors: &mut Vec<Entity>,
) {
    // Check same color neighbors from each direction
    let neighbor_positions = [
        BlockPosition::new(pos.x, pos.y - 1),
        BlockPosition::new(pos.x - 1, pos.y),
        BlockPosition::new(pos.x, pos.y + 1),
        BlockPosition::new(pos.x + 1, pos.y),
    ];
    for pos in neighbor_positions {
        if let Some(block) = block_map.get_block(&pos) {
            if block.1 == color {
                if !neighbors.contains(&block.0) {
                    neighbors.push(block.0);
                    find_same_color_neighbors(block_map, &pos, color, &mut neighbors)
                }
            }
        }
    }
}

/// Update board whenever new solid block is spawned
fn update_board(
    mut block_map: ResMut<BlockMap>,
    mut query: Query<
        (Entity, &BlockPosition, &BlockColor, &mut Number, &Operation),
        Added<SolidBlock>,
    >,
    mut events: EventWriter<PerformCalculationEvent>,
    mut last_dropped_block: ResMut<LastDroppedBlock>,
) {
    for (entity, pos, color, number, operation) in query.iter_mut() {
        // Find neighbors and send events to perform calculations in them
        let mut neighbors = Vec::<Entity>::new();
        find_same_color_neighbors(&block_map, &pos, color.clone(), &mut neighbors);

        // Send events
        for entity in &neighbors {
            events.send(PerformCalculationEvent {
                entity: *entity,
                number: number.0,
                operation: *operation,
            });
        }

        // Set last dropped block
        last_dropped_block.0 = Some(entity);

        // Add the spawned solid block into the BlockMap
        block_map.0.insert(*pos, Some((entity, *color)));
        block_map.print_debug();
    }
}
