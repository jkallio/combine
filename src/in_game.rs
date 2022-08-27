use crate::audio::{PlaySfxEvent, Sfx};
use crate::board::{BlockMap, BoardPlugin};
use crate::constants::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

/// This `Component` defines the math operation which the block performs
#[derive(Component, Copy, Clone)]
pub enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

/// This `Event` is sent when edge blocks need to be spawned
pub struct SpawnEdgeBlockEvent {
    position: Coords,
}

/// This `Event` is sent when new solid block needs to be spawned
pub struct SpawnDroppingBlockEvent {
    number: i32,
    position: Coords,
    color: BlockColor,
    operation: Operation,
}

/// This `Event` is sent when new dropping block needs to be spawned
pub struct SpawnSolidBlockEvent {
    number: i32,
    position: Coords,
    color: BlockColor,
}

/// This `Event` is sent when parameters for dropping block needs to be randomized
pub struct RandomizeDroppingBlockEvent;

/// This `Event` is sent when calculations needs to be performed for solid block
pub struct PerformCalculationEvent {
    pub entity: Entity,
    pub number: i32,
    pub operation: Operation,
}

/// This `Event` is sent when block number needs to be updated
pub struct UpdateBlockNumberEvent {
    entity: Entity,
    number: i32,
}

/// `Timer` for calculating when dropping block drops one square
struct DropTimer(Timer);

/// `Timer` for restricting horizontal movement
struct MoveTimer(Stopwatch);

/// Block image texture and text style is preloaded in this resource
struct BlockStyle {
    text_style: TextStyle,
    block: Handle<Image>,
    edge: Handle<Image>,
}

/// This resource holds the current block drop speed (in seconds)
struct DropSpeed(pub f32);

/// This resource holds the entity of the last dropped block
#[derive(Default)]
pub struct LastDroppedBlock {
    pub entity: Option<Entity>,
    pub operation: Option<Operation>,
}

/// All game objects are tagged with this `Component` for easier clean-up
#[derive(Component)]
pub struct GameObject;

/// Dropping block is tagged with this `Component`
#[derive(Component)]
pub struct DroppingBlock;

/// Dropped blocks are tagged with this `Component`
#[derive(Component)]
pub struct SolidBlock;

/// Edge blocks are tagged with this `Component`
#[derive(Component)]
pub struct EdgeBlock;

/// This `Component` determines the block number
#[derive(Component)]
pub struct Number(pub i32);

/// This `Component` holds the block color
#[derive(Component, PartialEq, Clone, Copy)]
pub enum BlockColor {
    BLUE,
    YELLOW,
    PINK,
    GREEN,
}

/// This `Component` determines the coordinates in `BlockMap`
#[derive(Component)]
pub struct BlockPosition(pub Coords);

/// Helper function to translate the `BlockColor` enum into actual RGB Color value
pub fn get_color(block_color: BlockColor) -> Color {
    match block_color {
        BlockColor::BLUE => Color::CYAN,
        BlockColor::YELLOW => Color::GOLD,
        BlockColor::PINK => Color::ORANGE_RED,
        BlockColor::GREEN => Color::LIME_GREEN,
    }
}

/// Bevy `Plugin` for handling the actual gameplay of this game
pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BoardPlugin)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(back_to_menu_on_esc),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(handle_dropping_block_movement),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_block_translation),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_edge_block))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_solid_block))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(spawn_dropping_block),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(randomize_new_block),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(perform_calculation),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_block_number),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_block_number_text),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_block_operation_text),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(switch_operation))
            .add_event::<SpawnEdgeBlockEvent>()
            .add_event::<SpawnSolidBlockEvent>()
            .add_event::<SpawnDroppingBlockEvent>()
            .add_event::<RandomizeDroppingBlockEvent>()
            .add_event::<PerformCalculationEvent>()
            .add_event::<UpdateBlockNumberEvent>()
            .insert_resource(DropTimer(Timer::from_seconds(INITIAL_DROP_SPEED, true)))
            .insert_resource(MoveTimer(Stopwatch::new()))
            .insert_resource(DropSpeed(INITIAL_DROP_SPEED))
            .insert_resource(LastDroppedBlock::default());
    }
}

/// Called once as the game is started
fn on_enter(
    mut commands: Commands,
    mut gen_event: EventWriter<RandomizeDroppingBlockEvent>,
    mut block_event: EventWriter<SpawnEdgeBlockEvent>,
    asset_server: Res<AssetServer>,
) {
    println!("Enter GameState::InGame");

    for y in -1..=BOARD_SIZE.height as i32 {
        for x in -1..=BOARD_SIZE.width as i32 {
            if y == -1 || y == BOARD_SIZE.height as i32 || x == -1 || x == BOARD_SIZE.width as i32 {
                block_event.send(SpawnEdgeBlockEvent {
                    position: Coords::new(x, y),
                });
            }
        }
    }
    let font = asset_server.load("fonts/04b_30.ttf");
    let block = asset_server.load("pixel-block.png");
    let edge = asset_server.load("edge-block.png");
    commands.insert_resource(BlockStyle {
        text_style: TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
        block,
        edge,
    });
    gen_event.send(RandomizeDroppingBlockEvent);
}

/// Called once after game has ended
fn on_exit(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    println!("Exit GameState::InGame");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System for returning to Menu in case Esc key is pressed
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

/// System for randomizing new parameters for the dropping block
fn randomize_new_block(
    mut gen_event: EventReader<RandomizeDroppingBlockEvent>,
    mut spawn_event: EventWriter<SpawnDroppingBlockEvent>,
) {
    for _ in gen_event.iter() {
        let num = rand::thread_rng().gen_range(0..=9);
        let col = rand::thread_rng().gen_range(1..=4);

        let color = match col {
            1 => BlockColor::BLUE,
            2 => BlockColor::YELLOW,
            3 => BlockColor::PINK,
            4 => BlockColor::GREEN,
            _ => {
                panic!("Invalid color num");
            }
        };

        spawn_event.send(SpawnDroppingBlockEvent {
            number: num,
            position: INITIAL_POSITION,
            color,
            operation: Operation::ADD,
        });
    }
}

/// System for spawning edge blocks
fn spawn_edge_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEdgeBlockEvent>,
    win_size: Res<WindowSize>,
    block_style: Res<BlockStyle>,
) {
    for ev in event_reader.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: block_style.edge.clone(),
                transform: Transform::from_xyz(
                    -win_size.0.x / 2.5 + ev.position.x as f32 * BLOCK_SIZE,
                    -win_size.0.y / 2.0 + ev.position.y as f32 * BLOCK_SIZE,
                    1.0,
                ),
                ..default()
            })
            .insert(GameObject)
            .insert(EdgeBlock);
    }
}

/// System for spawning solid blocks
fn spawn_solid_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnSolidBlockEvent>,
    win_size: Res<WindowSize>,
    block_style: Res<BlockStyle>,
) {
    for ev in event_reader.iter() {
        let block = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: get_color(ev.color),
                    ..default()
                },
                texture: block_style.block.clone(),
                transform: Transform::from_xyz(
                    -win_size.0.x / 2.5 + ev.position.x as f32 * BLOCK_SIZE,
                    -win_size.0.y / 2.0 + ev.position.y as f32 * BLOCK_SIZE,
                    1.0,
                ),
                ..default()
            })
            .insert(GameObject)
            .insert(SolidBlock)
            .insert(Number(ev.number))
            .insert(ev.color)
            .insert(BlockPosition(ev.position))
            .id();

        let text = commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(ev.number.to_string(), block_style.text_style.clone())
                    .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            })
            .id();

        commands.entity(block).push_children(&[text]);
    }
}

/// System for spawning dropping blocks
fn spawn_dropping_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnDroppingBlockEvent>,
    block_style: Res<BlockStyle>,
) {
    for ev in event_reader.iter() {
        let block = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: get_color(ev.color),
                    ..default()
                },
                texture: block_style.block.clone(),
                transform: Transform::from_xyz(
                    5.0 * BLOCK_SIZE,
                    (BOARD_SIZE.height as i32 - 1) as f32 * BLOCK_SIZE,
                    1.0,
                ),
                ..default()
            })
            .insert(GameObject)
            .insert(DroppingBlock)
            .insert(Number(ev.number))
            .insert(BlockPosition(ev.position))
            .insert(ev.color)
            .insert(ev.operation)
            .id();

        let text = commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(ev.number.to_string(), block_style.text_style.clone())
                    .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            })
            .id();

        commands.entity(block).push_children(&[text]);
    }
}

/// System for handling both player input and dropping block downward movement
fn handle_dropping_block_movement(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut drop_timer: ResMut<DropTimer>,
    mut time_since_last_moved: ResMut<MoveTimer>,
    mut query: Query<
        (Entity, &Number, &BlockColor, &mut BlockPosition, &Operation),
        With<DroppingBlock>,
    >,
    mut spawn_event: EventWriter<SpawnSolidBlockEvent>,
    mut gen_event: EventWriter<RandomizeDroppingBlockEvent>,
    mut drop_speed: ResMut<DropSpeed>,
    block_map: Res<BlockMap>,
    mut audio_events: EventWriter<PlaySfxEvent>,
    mut last_dropped_block: ResMut<LastDroppedBlock>,
) {
    for (entity, number, color, mut pos, op) in query.iter_mut() {
        // Handle Left / Right Movement
        // Block should move immediately after releasing the key or in case key is pressed move once
        // per 0.3 seconds.
        time_since_last_moved.0.tick(time.delta());
        if input.just_pressed(KeyCode::Left)
            || input.pressed(KeyCode::Left) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if block_map.is_none(&Coords::new(pos.0.x - 1, pos.0.y)) {
                pos.0.x -= 1;
            }
            time_since_last_moved.0.reset();
        } else if input.just_pressed(KeyCode::Right)
            || input.pressed(KeyCode::Right) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if block_map.is_none(&Coords::new(pos.0.x + 1, pos.0.y)) {
                pos.0.x += 1;
            }
            time_since_last_moved.0.reset();
        }

        // Handle block dropping
        drop_timer.0.tick(time.delta());
        if drop_timer.0.just_finished()
            || (drop_timer.0.elapsed_secs() >= 0.02 && (input.pressed(KeyCode::Down)))
        {
            if block_map.is_none(&Coords::new(pos.0.x, pos.0.y - 1)) {
                pos.0.y -= 1;
                drop_timer.0.reset();
            } else if pos.0.y >= INITIAL_POSITION.y {
                println!("GAME OVER!");
                // TODO: Move to Game Over Screen
            } else {
                // Spawn solid block where the dropping block ended
                spawn_event.send(SpawnSolidBlockEvent {
                    number: number.0,
                    position: Coords::new(pos.0.x, pos.0.y),
                    color: color.clone(),
                });

                // Store the last dropped block information
                last_dropped_block.entity = Some(entity);
                last_dropped_block.operation = Some(*op);

                // Despawn dropping block
                commands.entity(entity).despawn_recursive();
                audio_events.send(PlaySfxEvent(Sfx::BlockDropped));

                // Generate new dropping block
                gen_event.send(RandomizeDroppingBlockEvent);

                drop_speed.0 *= 0.99;
                drop_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(drop_speed.0));
            }
        }
    }
}

/// System for switching the math operation of the dropping block
fn switch_operation(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Operation, With<DroppingBlock>>,
) {
    if input.just_pressed(KeyCode::LShift) || input.just_pressed(KeyCode::RShift) {
        for mut op in query.iter_mut() {
            *op = match *op {
                Operation::ADD => Operation::SUBTRACT,
                Operation::SUBTRACT => Operation::MULTIPLY,
                Operation::MULTIPLY => Operation::DIVIDE,
                Operation::DIVIDE => Operation::ADD,
            }
        }
    }

    if input.just_pressed(KeyCode::Q) {
        if let Ok(mut op) = query.get_single_mut() {
            *op = Operation::ADD;
        }
    }
    if input.just_pressed(KeyCode::W) {
        if let Ok(mut op) = query.get_single_mut() {
            *op = Operation::SUBTRACT;
        }
    }
    if input.just_pressed(KeyCode::E) {
        if let Ok(mut op) = query.get_single_mut() {
            *op = Operation::MULTIPLY;
        }
    }
    if input.just_pressed(KeyCode::R) {
        if let Ok(mut op) = query.get_single_mut() {
            *op = Operation::DIVIDE;
        }
    }
}

/// System which is called to perform math calculation for each same color solid block
/// than the dropped block (next to dropping position)
pub fn perform_calculation(
    mut calculate_event: EventReader<PerformCalculationEvent>,
    mut query: Query<&Number, With<SolidBlock>>,
    mut last_dropped_block: ResMut<LastDroppedBlock>,
    mut update_num_event: EventWriter<UpdateBlockNumberEvent>,
) {
    for ev in calculate_event.iter() {
        if let Ok(number) = query.get_mut(ev.entity) {
            let number = match ev.operation {
                Operation::ADD => number.0 + ev.number,
                Operation::SUBTRACT => number.0 - ev.number,
                Operation::MULTIPLY => number.0 * ev.number,
                Operation::DIVIDE => (number.0 as f32 / ev.number as f32).ceil() as i32,
            };
            update_num_event.send(UpdateBlockNumberEvent {
                entity: ev.entity,
                number,
            });

            // Update also the last dropped block number
            if let Some(entity) = last_dropped_block.entity {
                update_num_event.send(UpdateBlockNumberEvent { entity, number });
                last_dropped_block.entity = None;
                last_dropped_block.operation = None;
            }
        }
    }
}

/// System for updating the block's actual translation (based on `BlockMap` position)
fn update_block_translation(
    win_size: Res<WindowSize>,
    mut query: Query<(&BlockPosition, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = -win_size.0.x / 2.5 + pos.0.x as f32 * BLOCK_SIZE;
        transform.translation.y = -win_size.0.y / 2.0 + pos.0.y as f32 * BLOCK_SIZE;
    }
}

/// System for handling the number change of a block
fn update_block_number(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Number, &BlockPosition)>,
    mut event: EventReader<UpdateBlockNumberEvent>,
    mut block_map: ResMut<BlockMap>,
    mut audio_events: EventWriter<PlaySfxEvent>,
) {
    for ev in event.iter() {
        if let Ok((entity, mut number, pos)) = query.get_mut(ev.entity) {
            number.0 = ev.number;
            if number.0 == 0 {
                commands.entity(entity).despawn_recursive();
                block_map.set_block(&pos.0, None);
                audio_events.send(PlaySfxEvent(Sfx::BlocksCleared));
            }
        }
    }
}

/// System for updating the `Text` child element of the block each time `Number` changes
pub fn update_block_number_text(
    mut block_query: Query<(&Number, &Children), Changed<Number>>,
    mut child_query: Query<&mut Text>,
) {
    for (number, children) in block_query.iter_mut() {
        for child in children {
            if let Ok(mut text) = child_query.get_mut(*child) {
                text.sections[0].value = number.0.to_string();
            }
        }
    }
}

/// System for updating the math operation character of the dropping block
/// TODO: Should only be called if operation is changed (and not every frame)
pub fn update_block_operation_text(
    mut block_query: Query<(&Number, &Children, &Operation, &DroppingBlock)>,
    mut child_query: Query<&mut Text>,
) {
    for (number, children, operation, _) in block_query.iter_mut() {
        for child in children {
            if let Ok(mut text) = child_query.get_mut(*child) {
                match operation {
                    Operation::ADD => {
                        text.sections[0].value = format!("+{}", number.0);
                    }
                    Operation::SUBTRACT => {
                        text.sections[0].value = format!("-{}", number.0);
                    }
                    Operation::MULTIPLY => {
                        text.sections[0].value = format!("x{}", number.0);
                    }
                    Operation::DIVIDE => {
                        text.sections[0].value = format!("/{}", number.0);
                    }
                }
            }
        }
    }
}
