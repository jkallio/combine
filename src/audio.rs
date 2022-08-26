use bevy::audio::{Audio, AudioSink};
use bevy::prelude::*;

/// Enum for PlaySfxEvent
pub enum Sfx {
    BlockDropped,
    BlocksCleared,
}

/// Event for playing sfx
pub struct PlaySfxEvent(pub Sfx);

/// Holds handles to game music and sfx samples
#[derive(Default, Clone)]
struct AudioResources {
    current_track: Handle<AudioSink>,
    tracks: Vec<Handle<AudioSource>>,
    sfx_drop: Handle<AudioSource>,
    sfx_clear: Handle<AudioSource>,
}

/// Bevy Plugin for handling music and sfx in the game
pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(mute_on_m_key)
            .add_system(change_background_track)
            .add_system(play_sfx)
            .insert_resource(AudioResources::default())
            .add_event::<PlaySfxEvent>();
    }
}

/// Startup system for loading and preparing audio assets
fn setup(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut audio_res: ResMut<AudioResources>,
    sinks: Res<Assets<AudioSink>>,
) {
    // Load sound effects
    audio_res.sfx_drop = asset_server.load("sounds/drop.ogg");
    audio_res.sfx_clear = asset_server.load("sounds/clear.ogg");

    // Load background music
    let track1 = asset_server.load("sounds/pixel-drama.ogg");
    audio_res.tracks.push(track1.clone());

    let track2 = asset_server.load("sounds/the-triumph-of-the-clockmaker.ogg");
    audio_res.tracks.push(track2);

    let track3 = asset_server.load("sounds/chamber-of-jewels.ogg");
    audio_res.tracks.push(track3);

    // play function returns weak handle which needs to be changed into strong handle which then
    // can be used for controlling the playback
    let track1_weak_handle =
        audio.play_with_settings(track1, PlaybackSettings::LOOP.with_volume(0.8));
    audio_res.current_track = sinks.get_handle(track1_weak_handle);
}

/// Toggle mute with M key
fn mute_on_m_key(
    input: Res<Input<KeyCode>>,
    sinks: Res<Assets<AudioSink>>,
    audio_res: Res<AudioResources>,
) {
    if input.just_pressed(KeyCode::M) {
        if let Some(sink) = sinks.get(&audio_res.current_track) {
            if sink.is_paused() {
                sink.play();
            } else {
                sink.pause();
            }
        }
    }
}

/// Change the background track
fn change_background_track(
    input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    sinks: Res<Assets<AudioSink>>,
    mut audio_res: ResMut<AudioResources>,
) {
    if input.any_just_pressed([KeyCode::Key1, KeyCode::Key2, KeyCode::Key3]) {
        if let Some(sink) = sinks.get(&audio_res.current_track) {
            sink.pause();
        }

        if input.just_pressed(KeyCode::Key1) {
            let track1_weak_handle = audio.play_with_settings(
                audio_res.tracks.get(0).unwrap().clone(),
                PlaybackSettings::LOOP.with_volume(0.8),
            );
            audio_res.current_track = sinks.get_handle(track1_weak_handle);
        }
        if input.just_pressed(KeyCode::Key2) {
            let track2_weak_handle = audio.play_with_settings(
                audio_res.tracks.get(1).unwrap().clone(),
                PlaybackSettings::LOOP.with_volume(0.8),
            );
            audio_res.current_track = sinks.get_handle(track2_weak_handle);
        }
        if input.just_pressed(KeyCode::Key3) {
            let track3_weak_handle = audio.play_with_settings(
                audio_res.tracks.get(2).unwrap().clone(),
                PlaybackSettings::LOOP.with_volume(0.8),
            );
            audio_res.current_track = sinks.get_handle(track3_weak_handle);
        }
    }
}

/// Receives PlaySfx events and plays the sample
fn play_sfx(
    mut events: EventReader<PlaySfxEvent>,
    audio_samples: Res<AudioResources>,
    audio: Res<Audio>,
) {
    for ev in events.iter() {
        match &ev.0 {
            Sfx::BlockDropped => {
                audio.play(audio_samples.sfx_drop.clone());
            }
            Sfx::BlocksCleared => {
                audio.play_with_settings(
                    audio_samples.sfx_clear.clone(),
                    PlaybackSettings::ONCE.with_volume(0.6),
                );
            }
        }
    }
}
