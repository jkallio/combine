use crate::prelude::*;
use bevy::prelude::*;
use bevy::window::close_on_esc;

/// Identifier for the Press Start
#[derive(Component)]
struct PressStartText;

/// Bevy Plugin for handling the game Main Menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(on_exit))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(start_game_on_enter))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(close_on_esc))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(blink_text));
    }
}

/// Called once when switching to `GameState::Menu`
fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Enter GameState::Menu");

    let menu = commands
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
        .insert(MenuNode)
        .id();

    // Press Start text
    let text = commands
        .spawn_bundle(
            TextBundle::from_section(
                "Press Start",
                TextStyle {
                    font: asset_server.load("fonts/04b_30.ttf"),
                    font_size: 32.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                position_type: PositionType::Relative,
                ..default()
            }),
        )
        .insert(PressStartText)
        .id();

    // Logo picture
    let logo = commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Auto),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(150.0),
                    bottom: Val::Auto,
                },
                align_self: AlignSelf::Center,
                ..default()
            },
            image: asset_server.load("logo.png").into(),
            ..default()
        })
        .id();
    commands.entity(menu).push_children(&[logo, text]);
}

/// Called once when switching from `GameState::Menu`
fn on_exit(mut commands: Commands, query: Query<Entity, With<MenuNode>>) {
    println!("Exit GameState::Menu");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Switch to `GameState::InGame` if Return key is pressed
fn start_game_on_enter(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Return) {
        game_state
            .set(GameState::HowToPlay)
            .expect("Failed to change GameState:InGame");
        input.reset(KeyCode::Return);
    }
}

/// Blink Press Start text
fn blink_text(time: Res<Time>, mut query: Query<&mut Text, With<PressStartText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        text.sections[0].style.color = Color::rgba(0., 0., 0., seconds.sin().abs());
    }
}
