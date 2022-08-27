use crate::prelude::*;
use bevy::prelude::*;
use bevy::window::close_on_esc;

/// Component identifier all Menu items
#[derive(Component)]
struct MenuItems;

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
        .insert(MenuItems)
        .id();

    let mut text_bundle = TextBundle::from_section(
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
    });

    text_bundle.transform = Transform::from_xyz(
        text_bundle.transform.translation.x,
        text_bundle.transform.translation.y,
        100.0,
    );

    let text = commands
        .spawn_bundle(text_bundle)
        .insert(PressStartText)
        .id();

    commands.entity(menu).push_children(&[text]);
}

/// Called once when switching from `GameState::Menu`
fn on_exit(mut commands: Commands, query: Query<Entity, With<MenuItems>>) {
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
            .set(GameState::InGame)
            .expect("Failed to change GameState:InGame");
        input.reset(KeyCode::Return);
    }
}

/// Blink Press Start text
fn blink_text(time: Res<Time>, mut query: Query<&mut Text, With<PressStartText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;

        text.sections[0].style.color = Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            //red: (1.25 * seconds).sin() / 2.0 + 0.5,
            //green: (0.75 * seconds).sin() / 2.0 + 0.5,
            //blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: seconds.sin().abs(),
        };
    }
}
