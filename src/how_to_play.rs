use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct ImgInstructions;

struct ReadyToPlay(bool);

/// Bevy plugin for showing how-to-play
pub struct HowToPlayPlugin;
impl Plugin for HowToPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::HowToPlay).with_system(on_enter))
            .add_system_set(SystemSet::on_exit(GameState::HowToPlay).with_system(on_exit))
            .add_system_set(
                SystemSet::on_update(GameState::HowToPlay).with_system(start_game_on_enter),
            )
            .insert_resource(ReadyToPlay(false));
    }
}

/// Called once when switching to `GameState::Menu`
fn on_enter(mut commands: Commands, my_assets: Res<MyAssets>) {
    println!("Enter GameState::HowToPlay");

    let node = commands
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

    let img = commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Auto),
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
            image: UiImage(my_assets.how_to_play_1_texture.clone()),
            ..default()
        })
        .insert(ImgInstructions)
        .id();

    commands.entity(node).push_children(&[img]);
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
    mut query: Query<&mut UiImage, With<ImgInstructions>>,
    mut ready_to_play: ResMut<ReadyToPlay>,
    my_assets: Res<MyAssets>,
) {
    if input.just_pressed(KeyCode::Return) {
        if ready_to_play.0 == true {
            game_state
                .set(GameState::InGame)
                .expect("Failed to change GameState:InGame");
            input.reset(KeyCode::Return);
            ready_to_play.0 = false;
        } else if let Ok(mut img) = query.get_single_mut() {
            img.0 = my_assets.how_to_play_2_texture.clone();
            ready_to_play.0 = true;
        }
    }
}
