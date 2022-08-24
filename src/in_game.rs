use crate::board::{BlockMap, BoardPlugin, BOARD_SIZE};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

const INITIAL_DROP_SPEED: f32 = 1.0;
const INITIAL_POSITION: BlockPosition = BlockPosition {
    x: 4,
    y: BOARD_SIZE.height as i32 - 1,
};

#[derive(Component, Copy, Clone)]
pub enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

/// Events
pub struct SpawnBlockEvent {
    is_dropping: bool,
    number: i32,
    position: BlockPosition,
    color: BlockColor,
    operation: Operation,
}

pub struct GenerateNewBlockEvent;
pub struct PerformCalculationEvent {
    pub entity: Entity,
    pub number: i32,
    pub operation: Operation,
}

pub struct UpdateBlockNumber {
    entity: Entity,
    number: i32,
}

/// Resources
struct DropTimer(Timer);
struct MoveTimer(Stopwatch);
struct BlockTextStyle(TextStyle);
struct DropSpeed(pub f32);
pub struct LastDroppedBlock(pub Option<Entity>);

/// Components
#[derive(Component)]
pub struct GameObject;
#[derive(Component)]
pub struct DroppingBlock;
#[derive(Component)]
pub struct SolidBlock;
#[derive(Component)]
pub struct Number(pub i32);
#[derive(Component, PartialEq, Clone, Copy)]
pub enum BlockColor {
    BLUE,
    YELLOW,
    PINK,
    GREEN,
}

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
}

impl BlockPosition {
    pub fn new(x: i32, y: i32) -> BlockPosition {
        BlockPosition { x, y }
    }
}

pub fn get_color(block_color: BlockColor) -> Color {
    match block_color {
        BlockColor::BLUE => Color::rgb(0.53, 0.8, 0.92),
        BlockColor::YELLOW => Color::YELLOW,
        BlockColor::PINK => Color::PINK,
        BlockColor::GREEN => Color::GREEN,
    }
}

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
                SystemSet::on_update(GameState::InGame).with_system(update_block_position),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_block))
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
            .add_event::<SpawnBlockEvent>()
            .add_event::<GenerateNewBlockEvent>()
            .add_event::<PerformCalculationEvent>()
            .add_event::<UpdateBlockNumber>()
            .insert_resource(DropTimer(Timer::from_seconds(INITIAL_DROP_SPEED, true)))
            .insert_resource(MoveTimer(Stopwatch::new()))
            .insert_resource(DropSpeed(INITIAL_DROP_SPEED))
            .insert_resource(LastDroppedBlock(None));
    }
}

fn on_enter(
    mut commands: Commands,
    mut gen_event: EventWriter<GenerateNewBlockEvent>,
    asset_server: Res<AssetServer>,
) {
    println!("Enter GameState::InGame");

    let font = asset_server.load("fonts/04b_30.ttf");
    commands.insert_resource(BlockTextStyle(TextStyle {
        font,
        font_size: 24.0,
        color: Color::BLACK,
    }));

    gen_event.send(GenerateNewBlockEvent);
}

fn randomize_new_block(
    mut gen_event: EventReader<GenerateNewBlockEvent>,
    mut spawn_event: EventWriter<SpawnBlockEvent>,
) {
    for _ in gen_event.iter() {
        let num = rand::thread_rng().gen_range(1..=9);
        let col = rand::thread_rng().gen_range(1..=4);
        let op = rand::thread_rng().gen_range(1..=4);

        let color = match col {
            1 => BlockColor::BLUE,
            2 => BlockColor::YELLOW,
            3 => BlockColor::PINK,
            4 => BlockColor::GREEN,
            _ => {
                panic!("Invalid color num");
            }
        };

        let operation = match op {
            1 => Operation::ADD,
            2 => Operation::SUBTRACT,
            3 => Operation::MULTIPLY,
            4 => Operation::DIVIDE,
            _ => {
                panic!("invalid operation");
            }
        };

        spawn_event.send(SpawnBlockEvent {
            is_dropping: true,
            number: num,
            position: INITIAL_POSITION,
            color,
            operation,
        });
    }
}

fn spawn_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnBlockEvent>,
    win_size: Res<WindowSize>,
    text_style: Res<BlockTextStyle>,
    asset_server: Res<AssetServer>,
) {
    for ev in event_reader.iter() {
        let transform = if ev.is_dropping {
            Transform::from_xyz(
                5.0 * BLOCK_SIZE,
                (BOARD_SIZE.height as i32 - 1) as f32 * BLOCK_SIZE,
                1.0,
            )
        } else {
            Transform::from_xyz(
                -win_size.0.x / 2.5 + ev.position.x as f32 * BLOCK_SIZE,
                -win_size.0.y / 2.0 + ev.position.y as f32 * BLOCK_SIZE,
                1.0,
            )
        };

        let block = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: get_color(ev.color),
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..default()
                },

                texture: asset_server.load("pixel-block.png"),
                transform,
                ..default()
            })
            .insert(GameObject)
            .insert(ev.position)
            .insert(Number(ev.number))
            .insert(ev.color)
            .insert(ev.operation)
            .id();

        if ev.is_dropping {
            commands.entity(block).insert(DroppingBlock);
        } else {
            commands.entity(block).insert(SolidBlock);
        }

        let text = commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(ev.number.to_string(), text_style.0.clone())
                    .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            })
            .id();

        commands.entity(block).push_children(&[text]);
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    println!("Exit GameState::InGame");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

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
    mut spawn_event: EventWriter<SpawnBlockEvent>,
    mut gen_event: EventWriter<GenerateNewBlockEvent>,
    block_map: Res<BlockMap>,
    mut drop_speed: ResMut<DropSpeed>,
) {
    for (entity, number, color, mut pos, operation) in query.iter_mut() {
        // Handle Left / Right Movement
        // Block should move immediately after releasing the key or in case key is pressed move once
        // per 0.3 seconds.
        time_since_last_moved.0.tick(time.delta());
        if input.just_pressed(KeyCode::Left)
            || input.pressed(KeyCode::Left) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if block_map.is_none(&BlockPosition::new(pos.x - 1, pos.y)) {
                pos.x = pos.x - 1;
            }
            time_since_last_moved.0.reset();
        } else if input.just_pressed(KeyCode::Right)
            || input.pressed(KeyCode::Right) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if block_map.is_none(&BlockPosition::new(pos.x + 1, pos.y)) {
                pos.x = pos.x + 1;
            }
            time_since_last_moved.0.reset();
        }

        // Handle block dropping
        drop_timer.0.tick(time.delta());
        if drop_timer.0.just_finished()
            || (drop_timer.0.elapsed_secs() >= 0.02 && (input.pressed(KeyCode::Down)))
        {
            if block_map.is_none(&BlockPosition::new(pos.x, pos.y - 1)) {
                pos.y -= 1;
                drop_timer.0.reset();
            } else {
                // Spawn solid block where the dropping block ended
                spawn_event.send(SpawnBlockEvent {
                    is_dropping: false,
                    number: number.0,
                    position: BlockPosition::new(pos.x, pos.y),
                    color: color.clone(),
                    operation: *operation,
                });

                // Despawn dropping block
                commands.entity(entity).despawn_recursive();

                // Generate new dropping block
                gen_event.send(GenerateNewBlockEvent);

                drop_speed.0 *= 0.99;
                drop_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(drop_speed.0));
            }
        }
    }
}

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

pub fn perform_calculation(
    mut calculate_event: EventReader<PerformCalculationEvent>,
    mut query: Query<&Number, With<SolidBlock>>,
    mut last_dropped_block: ResMut<LastDroppedBlock>,
    mut update_num_event: EventWriter<UpdateBlockNumber>,
) {
    for ev in calculate_event.iter() {
        if let Ok(number) = query.get_mut(ev.entity) {
            let number = match ev.operation {
                Operation::ADD => number.0 + ev.number,
                Operation::SUBTRACT => number.0 - ev.number,
                Operation::MULTIPLY => number.0 * ev.number,
                Operation::DIVIDE => (number.0 as f32 / ev.number as f32).ceil() as i32,
            };
            update_num_event.send(UpdateBlockNumber {
                entity: ev.entity,
                number,
            });

            // Update also the last dropped block number
            if let Some(entity) = last_dropped_block.0 {
                update_num_event.send(UpdateBlockNumber { entity, number });
                last_dropped_block.0 = None;
            }
        }
    }
}

fn update_block_position(
    win_size: Res<WindowSize>,
    mut query: Query<(&BlockPosition, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = -win_size.0.x / 2.5 + pos.x as f32 * BLOCK_SIZE;
        transform.translation.y = -win_size.0.y / 2.0 + pos.y as f32 * BLOCK_SIZE;
    }
}

fn update_block_number(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Number, &BlockPosition)>,
    mut event: EventReader<UpdateBlockNumber>,
    mut block_map: ResMut<BlockMap>,
) {
    for ev in event.iter() {
        if let Ok((entity, mut number, pos)) = query.get_mut(ev.entity) {
            number.0 = ev.number;
            if number.0 == 0 {
                commands.entity(entity).despawn_recursive();
                block_map.set_block(pos, None);
            }
        }
    }
}

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
