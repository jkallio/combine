use crate::board::{Board, BoardPlugin, BOARD_SIZE};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

const INITIAL_DROP_SPEED: f32 = 1.0;
const INITIAL_POSITION: BlockPosition = BlockPosition {
    x: 4,
    y: BOARD_SIZE.height as i32 - 1,
};

/// Events
pub struct SpawnBlockEvent {
    is_dropping: bool,
    number: i32,
    position: BlockPosition,
    color: Color,
}

pub struct GenerateNewBlockEvent;

/// Resources
struct DropTimer(Timer);
struct MoveTimer(Stopwatch);
struct BlockTextStyle(TextStyle);
struct DropSpeed(pub f32);

/// Components
#[derive(Component)]
pub struct GameObject;
#[derive(Component)]
pub struct DroppingBlock;
#[derive(Component)]
pub struct SolidBlock;
#[derive(Component)]
pub struct Number(pub i32);
#[derive(Component)]
struct BlockColor(Color);
#[derive(Component, Clone, Copy)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
}
impl BlockPosition {
    pub fn new(x: i32, y: i32) -> BlockPosition {
        BlockPosition { x, y }
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
                SystemSet::on_update(GameState::InGame).with_system(update_block_positions),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_block))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(generate_new_block))
            .add_event::<SpawnBlockEvent>()
            .add_event::<GenerateNewBlockEvent>()
            .insert_resource(DropTimer(Timer::from_seconds(INITIAL_DROP_SPEED, true)))
            .insert_resource(MoveTimer(Stopwatch::new()))
            .insert_resource(DropSpeed(INITIAL_DROP_SPEED));
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
        font_size: 32.0,
        color: Color::BLACK,
    }));

    gen_event.send(GenerateNewBlockEvent);
}

fn generate_new_block(
    mut gen_event: EventReader<GenerateNewBlockEvent>,
    mut spawn_event: EventWriter<SpawnBlockEvent>,
) {
    for _ in gen_event.iter() {
        let num = rand::thread_rng().gen_range(1..10);
        let col = rand::thread_rng().gen_range(1..5);

        let color = match col {
            1 => Color::BLUE,
            2 => Color::YELLOW,
            3 => Color::PINK,
            4 => Color::GREEN,
            _ => {
                panic!("Invalid color num");
            }
        };

        spawn_event.send(SpawnBlockEvent {
            is_dropping: true,
            number: num,
            position: INITIAL_POSITION,
            color,
        });
    }
}

fn spawn_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnBlockEvent>,
    win_size: Res<WindowSize>,
    text_style: Res<BlockTextStyle>,
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
                    color: ev.color,
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..default()
                },
                transform,
                ..default()
            })
            .insert(GameObject)
            .insert(ev.position)
            .insert(Number(ev.number))
            .insert(BlockColor(ev.color))
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
    mut query: Query<(Entity, &Number, &BlockColor, &mut BlockPosition), With<DroppingBlock>>,
    mut spawn_event: EventWriter<SpawnBlockEvent>,
    mut gen_event: EventWriter<GenerateNewBlockEvent>,
    board: Res<Board>,
    mut drop_speed: ResMut<DropSpeed>,
) {
    for (entity, number, color, mut pos) in query.iter_mut() {
        // Handle Left / Right Movement
        // Block should move immediately after releasing the key or in case key is pressed move once
        // per 0.3 seconds.
        time_since_last_moved.0.tick(time.delta());
        if input.just_pressed(KeyCode::Left)
            || input.pressed(KeyCode::Left) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if board.is_free(BlockPosition::new(pos.x - 1, pos.y)) {
                pos.x = pos.x - 1;
            }
            time_since_last_moved.0.reset();
        } else if input.just_pressed(KeyCode::Right)
            || input.pressed(KeyCode::Right) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if board.is_free(BlockPosition::new(pos.x + 1, pos.y)) {
                pos.x = pos.x + 1;
            }
            time_since_last_moved.0.reset();
        }

        // Handle block dropping
        drop_timer.0.tick(time.delta());
        if drop_timer.0.just_finished()
            || (drop_timer.0.elapsed_secs() >= 0.02 && (input.pressed(KeyCode::Down)))
        {
            if board.is_free(BlockPosition::new(pos.x, pos.y - 1)) {
                pos.y -= 1;
                drop_timer.0.reset();
            } else {
                // Spawn solid block where the dropping block ended
                spawn_event.send(SpawnBlockEvent {
                    is_dropping: false,
                    number: number.0,
                    position: BlockPosition::new(pos.x, pos.y),
                    color: color.0,
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

fn update_block_positions(
    win_size: Res<WindowSize>,
    mut query: Query<(&BlockPosition, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = -win_size.0.x / 2.5 + pos.x as f32 * BLOCK_SIZE;
        transform.translation.y = -win_size.0.y / 2.0 + pos.y as f32 * BLOCK_SIZE;
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
