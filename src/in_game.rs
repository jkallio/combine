use crate::audio::{PlaySfxEvent, Sfx};
use crate::board::{BlockMap, BoardPlugin, MoveBlockEvent};
use crate::constants::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;
use std::collections::HashMap;

/// This `Component` defines the math operation which the block performs
#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
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

/// This list contains "despawning" blocks not despawned immediately because of animation.
#[derive(Default)]
pub struct DespawningBlocks(HashMap<Entity, (Entity, Timer, Timer)>);

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

/// This resource holds the current block drop speed (in seconds)
struct DropSpeed(pub f32);

/// This resource holds the entity of the last dropped block
#[derive(Default)]
pub struct LastDroppedBlock {
    pub entity: Option<Entity>,
    pub operation: Option<Operation>,
}

/// Identifier for the HUD layer
#[derive(Component)]
pub struct HudLayer;

/// Identifier for the ScoreText
#[derive(Component)]
pub struct ScoreText;

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
    NONE,
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
        BlockColor::NONE => Color::NONE,
        BlockColor::BLUE => Color::CYAN,
        BlockColor::YELLOW => Color::GOLD,
        BlockColor::PINK => Color::ORANGE_RED,
        BlockColor::GREEN => Color::LIME_GREEN,
    }
}

/// Helper function to translate `Operation` into string
pub fn get_operator(op: Operation) -> String {
    match op {
        Operation::ADD => "+",
        Operation::SUBTRACT => "-",
        Operation::MULTIPLY => "x",
        Operation::DIVIDE => "/",
    }
    .to_string()
}

/// Helper function to get real position
pub fn get_translation(win_size: &Vec2, pos: &Coords) -> Vec3 {
    Vec3::new(
        -win_size.x / 2.0 + pos.x as f32 * BLOCK_SIZE + BLOCK_SIZE + 15.0,
        -win_size.y / 2.0 + pos.y as f32 * BLOCK_SIZE - HALF_BLOCK + 7.0,
        1.0,
    )
}

/// Bevy `Plugin` for handling the actual gameplay of this game
pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BoardPlugin)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(back_to_menu_on_esc)
                    .with_system(handle_dropping_block_movement)
                    .with_system(update_block_translation)
                    .with_system(spawn_solid_block)
                    .with_system(spawn_dropping_block)
                    .with_system(randomize_new_block)
                    .with_system(perform_calculation)
                    .with_system(update_block_number)
                    .with_system(update_block_number_text)
                    .with_system(update_score_text)
                    .with_system(update_operator_text)
                    .with_system(update_block_color)
                    .with_system(despawn_blocks)
                    .with_system(switch_dropping_block_color)
                    .with_system(drop_floating_blocks),
            )
            .add_event::<SpawnSolidBlockEvent>()
            .add_event::<SpawnDroppingBlockEvent>()
            .add_event::<RandomizeDroppingBlockEvent>()
            .add_event::<PerformCalculationEvent>()
            .add_event::<UpdateBlockNumberEvent>()
            .insert_resource(MoveTimer(Stopwatch::new()))
            .insert_resource(DropSpeed(INITIAL_DROP_SPEED))
            .insert_resource(LastDroppedBlock::default())
            .insert_resource(DespawningBlocks::default());
    }
}

/// Called once as the game is started
fn on_enter(
    mut commands: Commands,
    mut gen_event: EventWriter<RandomizeDroppingBlockEvent>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut drop_speed: ResMut<DropSpeed>,
) {
    println!("Enter GameState::InGame");

    // Spawn score text
    let hud = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(HudLayer)
        .insert(GameObject)
        .id();

    let text = commands
        .spawn_bundle(TextBundle::from_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/04b_30.ttf"),
                font_size: 32.0,
                color: Color::BLACK,
            },
        ))
        .insert(ScoreText)
        .id();

    commands.entity(hud).push_children(&[text]);

    // Generate the first dropping block
    gen_event.send(RandomizeDroppingBlockEvent);

    // Reset the score resource
    score.0 = 0;

    // Reset the initial drop speed
    drop_speed.0 = INITIAL_DROP_SPEED;
    commands.insert_resource(DropTimer(Timer::from_seconds(INITIAL_DROP_SPEED, true)))
}

/// Called once after game has ended
fn on_exit() {
    println!("Exit GameState::InGame");
}

/// System for returning to Menu in case Esc key is pressed
pub fn back_to_menu_on_esc(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) && *game_state.current() != GameState::Menu {
        game_state
            .set(GameState::GameOver)
            .expect("Failed to change GameState::GameOver");
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
        let op = rand::thread_rng().gen_range(1..=4);

        let color = match col {
            1 => BlockColor::BLUE,
            2 => BlockColor::YELLOW,
            3 => BlockColor::PINK,
            _ => BlockColor::GREEN,
        };

        let mut operation = match op {
            1 => Operation::ADD,
            2 => Operation::SUBTRACT,
            3 => Operation::MULTIPLY,
            _ => Operation::DIVIDE,
        };

        // Prevent division by zero
        if operation == Operation::DIVIDE && num == 0 {
            operation = Operation::MULTIPLY;
        }

        spawn_event.send(SpawnDroppingBlockEvent {
            number: num,
            position: INITIAL_POSITION,
            color,
            operation,
        });
    }
}

/// System for spawning solid blocks
fn spawn_solid_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnSolidBlockEvent>,
    block_style: Res<MyAssets>,
) {
    for ev in event_reader.iter() {
        let block = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: get_color(ev.color),
                    ..default()
                },
                texture: block_style.block_texture.clone(),
                transform: INITIAL_TRANSFORM,
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
    my_assets: Res<MyAssets>,
) {
    for ev in event_reader.iter() {
        let block = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: get_color(ev.color),
                    ..default()
                },
                texture: my_assets.block_texture.clone(),
                transform: INITIAL_TRANSFORM,
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
                text: Text::from_section(
                    format!("{}{}", get_operator(ev.operation), ev.number),
                    my_assets.text_style.clone(),
                )
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
    mut game_state: ResMut<State<GameState>>,
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

                game_state
                    .set(GameState::GameOver)
                    .expect("Failed to change GameState::GameOver");
            } else {
                // Spawn solid block where the dropping block ended
                spawn_event.send(SpawnSolidBlockEvent {
                    number: number.0,
                    position: Coords::new(pos.0.x, pos.0.y),
                    color: color.clone(),
                });

                // Store the last dropped block operation before it's despawned
                last_dropped_block.operation = Some(*op);

                // Despawn dropping block
                commands.entity(entity).despawn_recursive();
                audio_events.send(PlaySfxEvent(Sfx::BlockDropped));

                // Generate new dropping block
                gen_event.send(RandomizeDroppingBlockEvent);

                // Speed up a little each time block is droped
                drop_speed.0 *= 0.99;
                drop_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(drop_speed.0));
            }
        }
    }
}

/// System for switching the color of the dropping block
fn switch_dropping_block_color(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut BlockColor, With<DroppingBlock>>,
) {
    if let Ok(mut color) = query.get_single_mut() {
        if input.just_pressed(KeyCode::RShift) || input.just_pressed(KeyCode::LShift) {
            *color = match *color {
                BlockColor::NONE => BlockColor::NONE,
                BlockColor::BLUE => BlockColor::PINK,
                BlockColor::PINK => BlockColor::GREEN,
                BlockColor::GREEN => BlockColor::YELLOW,
                BlockColor::YELLOW => BlockColor::BLUE,
            }
        }
        if input.just_pressed(KeyCode::Q) {
            *color = BlockColor::BLUE;
        } else if input.just_pressed(KeyCode::W) {
            *color = BlockColor::PINK;
        } else if input.just_pressed(KeyCode::E) {
            *color = BlockColor::YELLOW;
        } else if input.just_pressed(KeyCode::R) {
            *color = BlockColor::GREEN;
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
                Operation::DIVIDE => {
                    if number.0 > 0 {
                        (number.0 as f32 / ev.number as f32).ceil() as i32
                    } else if number.0 < 0 {
                        (number.0 as f32 / ev.number as f32).floor() as i32
                    } else {
                        0
                    }
                }
            }
            .clamp(-99, 99);
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
    mut query: Query<(&BlockPosition, &mut Transform)>, //, Changed<BlockPosition>>,
) {
    // TODO: For some reason block translation would not always update if Changed-filter was being used.
    for (pos, mut transform) in query.iter_mut() {
        let trans = get_translation(&win_size.0, &pos.0);
        transform.translation.x = trans.x;
        transform.translation.y = trans.y;
    }
}

/// System for handling the number change of a block
fn update_block_number(
    mut query: Query<(Entity, &mut Number, &BlockPosition)>,
    mut event: EventReader<UpdateBlockNumberEvent>,
    mut block_map: ResMut<BlockMap>,
    mut audio_events: EventWriter<PlaySfxEvent>,
    mut despawning_blocks: ResMut<DespawningBlocks>,
    mut score: ResMut<Score>,
) {
    for ev in event.iter() {
        if let Ok((entity, mut number, pos)) = query.get_mut(ev.entity) {
            // Target reached --> Despawn block
            if ev.number % 10 == 0 {
                score.0 += number.0;
                despawning_blocks.0.insert(
                    entity,
                    (
                        entity,
                        Timer::from_seconds(1.0, false),
                        Timer::from_seconds(0.05, false),
                    ),
                );

                block_map.set_block(&pos.0, None);
                audio_events.send(PlaySfxEvent(Sfx::BlocksCleared));
            } else {
                number.0 = ev.number
            }
        }
    }
}

/// Despawn blocks
fn despawn_blocks(
    mut commands: Commands,
    time: Res<Time>,
    mut despawning_blocks: ResMut<DespawningBlocks>,
    mut query: Query<(&mut Sprite, &mut Visibility)>,
) {
    let mut removals = vec![];
    for block in despawning_blocks.0.values_mut() {
        if block.1.tick(time.delta()).just_finished() {
            commands.entity(block.0).despawn_recursive();
            removals.push(block.0);
        } else if block.2.tick(time.delta()).just_finished() {
            if let Ok((mut sprite, mut visibility)) = query.get_mut(block.0) {
                visibility.is_visible = if visibility.is_visible { false } else { true };
                sprite.color = Color::WHITE;
            }
            block.2.reset();
        }
    }

    for key in removals {
        despawning_blocks.0.remove(&key);
    }
}

/// System for updating the `Text` child element of the block each time `Number` changes
pub fn update_block_number_text(
    mut block_query: Query<(&Number, &Children, &SolidBlock), Changed<Number>>,
    mut child_query: Query<&mut Text>,
) {
    for (number, children, _) in block_query.iter_mut() {
        for child in children {
            if let Ok(mut text) = child_query.get_mut(*child) {
                text.sections[0].value = number.0.to_string();
            }
        }
    }
}
/// System for updating the math operation character of the dropping block
pub fn update_operator_text(
    mut block_query: Query<(&Number, &Children, &Operation, &DroppingBlock), Changed<Operation>>,
    mut child_query: Query<&mut Text>,
) {
    for (number, children, operation, _) in block_query.iter_mut() {
        for child in children {
            if let Ok(mut text) = child_query.get_mut(*child) {
                text.sections[0].value = format!("{}{}", get_operator(*operation), number.0);
            }
        }
    }
}

/// Update the block number color each time BlockColor changes
pub fn update_block_color(mut query: Query<(&BlockColor, &mut Sprite), Changed<BlockColor>>) {
    for (block_color, mut sprite) in query.iter_mut() {
        sprite.color = get_color(*block_color);
    }
}

/// Update the score text
pub fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("{}", score.0);
    }
}

/// Drop solid block if below square is empty
pub fn drop_floating_blocks(
    mut query: Query<&mut BlockPosition, With<SolidBlock>>,
    block_map: Res<BlockMap>,
    despawning_blocks: Res<DespawningBlocks>,
    mut event_writer: EventWriter<MoveBlockEvent>,
) {
    if despawning_blocks.0.is_empty() {
        for mut pos in query.iter_mut() {
            if block_map
                .get_block(&Coords::new(pos.0.x, pos.0.y - 1))
                .is_none()
            {
                let new_pos = Coords::new(pos.0.x, pos.0.y - 1);
                event_writer.send(MoveBlockEvent {
                    old_pos: pos.0,
                    new_pos,
                });
                pos.0 = new_pos;
            }
        }
    }
}
