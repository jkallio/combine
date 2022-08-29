use crate::in_game::HudLayer;
use crate::prelude::*;
use bevy::prelude::*;

/// Identifier for the background/overlay layer
#[derive(Component)]
struct GameOverBackground;

/// Identifier for the GameOver Text
#[derive(Component)]
struct GameOverText;

/// Bevy Plugin for handling Game Over state
pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(back_to_menu_on_enter),
            );
    }
}

/// Called once when Game is Over
fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
    mut query: Query<(Entity, &mut UiColor), With<HudLayer>>,
) {
    println!("Enter GameState::GameOver");

    let txt: String = if score.0 > high_score.0 {
        format!("GAME OVER\r\n\r\n* NEW HIGHSCORE *")
    } else {
        format!("GAME OVER\r\n\r\nHi: {}", high_score.0)
    };

    if score.0 > high_score.0 {
        println!("New high_score: {}", high_score.0);
        high_score.0 = score.0;
    }

    if let Ok((hud, mut color)) = query.get_single_mut() {
        *color = UiColor(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.95,
        });

        let text = commands
            .spawn_bundle(
                TextBundle::from_section(
                    txt,
                    TextStyle {
                        font: asset_server.load("fonts/04b_30.ttf"),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                }),
            )
            .insert(GameOverText)
            .id();
        commands.entity(hud).push_children(&[text]);
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Switch to `GameState::Menu` if Return key is pressed
fn back_to_menu_on_enter(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Return) || input.just_pressed(KeyCode::Escape) {
        game_state
            .set(GameState::Menu)
            .expect("Failed to change GameState:Menu");
        input.reset(KeyCode::Return);
    }
}
