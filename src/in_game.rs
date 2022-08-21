use crate::prelude::*;
use bevy::prelude::*;

pub struct SpawnBlockEvent;

#[derive(Component)]
pub struct GameObject;

#[derive(Component)]
pub struct DroppingBlock;

#[derive(Component)]
pub struct DropSpeed(pub f32);

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(back_to_menu_on_esc),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_dropping_block_movement),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(spawn_dropping_block),
            )
            .add_event::<SpawnBlockEvent>();
    }
}

fn on_enter(mut event_writer: EventWriter<SpawnBlockEvent>) {
    println!("Enter GameState::InGame");
    event_writer.send(SpawnBlockEvent);
}

fn spawn_dropping_block(mut commands: Commands, event_reader: EventReader<SpawnBlockEvent>) {
    if !event_reader.is_empty() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(DroppingBlock)
            .insert(GameObject)
            .insert(DropSpeed(-50.0));
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    println!("Exit GameState::InGame");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_dropping_block_movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &DropSpeed), With<DroppingBlock>>,
) {
    for (mut transform, drop_speed) in query.iter_mut() {
        if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
            transform.translation.x -= BLOCK_SIZE;
        } else if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::A) {
            transform.translation.x += BLOCK_SIZE;
        }

        let drop_multiplier;
        if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
            drop_multiplier = 3.0;
        } else {
            drop_multiplier = 1.0;
        }

        transform.translation.y += drop_speed.0 * drop_multiplier * time.delta_seconds();
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
