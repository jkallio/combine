use crate::constants::prelude::*;
use crate::in_game::{
    BlockColor, BlockPosition, LastDroppedBlock, Number, PerformCalculationEvent, SolidBlock,
};
use crate::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

/// Type definition for block information
type BlockInfo = Option<(Entity, BlockColor)>;

/// This event can be sent for moving a block in a map
pub struct MoveBlockEvent {
    pub old_pos: Coords,
    pub new_pos: Coords,
}

/// Represents the block map of all the dropped blocks
pub struct BlockMap(HashMap<Coords, BlockInfo>);
impl BlockMap {
    /// Returns new empty `BlockMap`
    pub fn new_empty() -> BlockMap {
        let block_map = HashMap::<Coords, BlockInfo>::new();
        BlockMap(block_map)
    }

    /// Retruns `true` if no block found from position
    pub fn is_none(&self, pos: &Coords) -> bool {
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

    /// Return block information from given position
    pub fn get_block(&self, pos: &Coords) -> BlockInfo {
        if let Some(value) = self.0.get(pos) {
            return value.clone();
        }
        None
    }

    /// Set block into given position
    pub fn set_block(&mut self, pos: &Coords, value: BlockInfo) {
        if value.is_some() {
            self.0.insert(*pos, value);
        } else if self.get_block(pos).is_some() {
            self.0.remove(pos);
        }
    }

    #[allow(dead_code)]
    pub fn debug_draw(&self) {
        for y in (0..BOARD_SIZE.height).rev() {
            for x in 0..BOARD_SIZE.width {
                if self.get_block(&Coords::new(x as i32, y as i32)).is_none() {
                    print!(". ");
                } else {
                    print!("X ");
                }
            }
            println!("");
        }
        println!("--------------");
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(handle_block_dropped)
                    .with_system(handle_moved_block),
            )
            .insert_resource(BlockMap::new_empty())
            .add_event::<MoveBlockEvent>();
    }
}

fn on_enter(
    mut block_map: ResMut<BlockMap>,
    query: Query<(Entity, &BlockPosition), With<EdgeBlock>>,
) {
    block_map.0.clear();

    for (entity, pos) in query.iter() {
        block_map.0.insert(pos.0, Some((entity, BlockColor::NONE)));
    }
}

fn on_exit(mut block_map: ResMut<BlockMap>) {
    block_map.0.clear();
}

/// Finds same colored neighbors by calling itself recursively for each found neighbor
fn find_same_color_neighbors(
    block_map: &BlockMap,
    pos: &Coords,
    color: BlockColor,
    mut neighbors: &mut Vec<Entity>,
) {
    // Check same color neighbors from each direction
    let neighbor_positions = [
        Coords::new(pos.x, pos.y - 1),
        Coords::new(pos.x - 1, pos.y),
        Coords::new(pos.x, pos.y + 1),
        Coords::new(pos.x + 1, pos.y),
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
fn handle_block_dropped(
    mut block_map: ResMut<BlockMap>,
    mut query: Query<(Entity, &BlockPosition, &BlockColor, &mut Number), Added<SolidBlock>>,
    mut events: EventWriter<PerformCalculationEvent>,
    mut last_dropped_block: ResMut<LastDroppedBlock>,
) {
    for (entity, pos, color, number) in query.iter_mut() {
        // Find neighbors and send events to perform calculations in them
        let mut neighbors = Vec::<Entity>::new();
        find_same_color_neighbors(&block_map, &pos.0, color.clone(), &mut neighbors);

        // Send events
        for entity in &neighbors {
            events.send(PerformCalculationEvent {
                entity: *entity,
                number: number.0,
                operation: last_dropped_block.operation.unwrap(),
            });
        }

        // Added Solid block is always also last dropped block
        last_dropped_block.entity = Some(entity);

        // Add the spawned solid block into the BlockMap
        block_map.0.insert(pos.0, Some((entity, *color)));
    }
}

fn handle_moved_block(
    mut block_map: ResMut<BlockMap>,
    mut event_reader: EventReader<MoveBlockEvent>,
) {
    for ev in event_reader.iter() {
        if let Some(block) = block_map.get_block(&ev.old_pos) {
            block_map.0.remove(&ev.old_pos);
            if block_map.get_block(&ev.new_pos).is_none() {
                block_map.0.insert(ev.new_pos, Some(block));
            } else {
                println!(
                    "Error!! Already occupied {}, {}",
                    ev.new_pos.x, ev.new_pos.y
                );
            }
        }
    }
}
