use crate::board::{Board, BoardPlugin, BOARD_SIZE};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::time::Stopwatch;

const INITIAL_DROP_SPEED: f32 = 1.0;

/// Events
pub struct SpawnDroppingBlockEvent {
    drop_speed: f32,
}
pub struct SpawnSolidBlockEvent {
    position: Position,
}

/// Resources
pub struct DropTimer(Timer);
pub struct MoveTimer(Stopwatch);

/// Components
#[derive(Component)]
pub struct GameObject;
#[derive(Component)]
pub struct DroppingBlock;
#[derive(Component)]
pub struct SolidBlock;
#[derive(Component)]
pub struct DropSpeed(pub f32);
#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
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
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(spawn_dropping_block),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(spawn_solid_block))
            .add_event::<SpawnDroppingBlockEvent>()
            .add_event::<SpawnSolidBlockEvent>()
            .insert_resource(DropTimer(Timer::from_seconds(INITIAL_DROP_SPEED, true)))
            .insert_resource(MoveTimer(Stopwatch::new()));
    }
}

fn on_enter(mut event_writer: EventWriter<SpawnDroppingBlockEvent>) {
    println!("Enter GameState::InGame");
    event_writer.send(SpawnDroppingBlockEvent {
        drop_speed: INITIAL_DROP_SPEED,
    });
}

fn spawn_solid_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnSolidBlockEvent>,
    win_size: Res<WindowSize>,
) {
    for ev in event_reader.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::PINK,
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    -win_size.0.x / 2.0 + ev.position.x as f32 * BLOCK_SIZE,
                    -win_size.0.y / 2.0 + ev.position.y as f32 * BLOCK_SIZE,
                    1.0,
                ),
                ..Default::default()
            })
            .insert(SolidBlock)
            .insert(GameObject)
            .insert(ev.position);
    }
}

fn spawn_dropping_block(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnDroppingBlockEvent>,
    mut game_state: ResMut<State<GameState>>,
    board: Res<Board>,
) {
    let pos = Position::new(5, BOARD_SIZE.height as i32 - 1);
    for ev in event_reader.iter() {
        if board.is_solid(pos) {
            println!("GAME OVER!");
            // TODO: Switch to GameOver screen
            game_state
                .set(GameState::Menu)
                .expect("Failed to set GameState");
        } else {
            println!("SpawnDroppingBlockEvent received.");
            board.print();
        }
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    pos.x as f32 * BLOCK_SIZE,
                    pos.y as f32 * BLOCK_SIZE,
                    1.0,
                ),
                ..Default::default()
            })
            .insert(DroppingBlock)
            .insert(GameObject)
            .insert(DropSpeed(ev.drop_speed))
            .insert(pos);
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
    mut query: Query<(Entity, &mut Position, &DropSpeed), With<DroppingBlock>>,
    mut drop_events: EventWriter<SpawnDroppingBlockEvent>,
    mut solid_events: EventWriter<SpawnSolidBlockEvent>,
    board: Res<Board>,
) {
    for (entity, mut pos, drop_speed) in query.iter_mut() {
        // Handle Left / Right Movement
        // Block should move immediately after releasing the key or in case key is pressed move once
        // per 0.3 seconds.
        time_since_last_moved.0.tick(time.delta());
        if input.just_pressed(KeyCode::Left)
            || input.pressed(KeyCode::Left) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if board.is_free(Position::new(pos.x - 1, pos.y)) {
                pos.x = pos.x - 1;
            }
            time_since_last_moved.0.reset();
        } else if input.just_pressed(KeyCode::Right)
            || input.pressed(KeyCode::Right) && time_since_last_moved.0.elapsed_secs() > 0.3
        {
            if board.is_free(Position::new(pos.x + 1, pos.y)) {
                pos.x = pos.x + 1;
            }
            time_since_last_moved.0.reset();
        }

        // Handle block dropping
        drop_timer.0.tick(time.delta());
        if drop_timer.0.just_finished()
            || (drop_timer.0.elapsed_secs() >= 0.02 && (input.pressed(KeyCode::Down)))
        {
            if board.is_free(Position::new(pos.x, pos.y - 1)) {
                pos.y -= 1;
                drop_timer.0.reset();
            } else {
                solid_events.send(SpawnSolidBlockEvent {
                    position: Position::new(pos.x, pos.y),
                });

                drop_events.send(SpawnDroppingBlockEvent {
                    drop_speed: drop_speed.0 * 0.99,
                });
                println!("Send SpawnDroppingBlockEvent");

                commands.entity(entity).despawn_recursive();
                drop_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(drop_speed.0 * 0.99));
            }
        }
    }
}

fn update_block_positions(
    win_size: Res<WindowSize>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = -win_size.0.x / 2.0 + pos.x as f32 * BLOCK_SIZE;
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
