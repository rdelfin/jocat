use crate::{
    audio,
    prefabs::ThrownPrefab,
    resources::{GameEvent, Level, LevelStart, ThrownPrefabSet},
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    audio::{output::Output, Source},
    derive::SystemDesc,
    ecs::{prelude::*, Read, ReadExpect, System},
};
use log::info;

// We're using 122 BPM, and this reduces it down to seconds per beat
const SECONDS_PER_BEAT: f64 = 60. / 122.;

// This system controlls the overall animations and starts them based on the audio cues
#[derive(Default, SystemDesc)]
pub struct TimingSystem {
    beat: u64,
}

impl<'s> System<'s> for TimingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'s>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, audio::Sounds>,
        Option<Read<'s, Output>>,
        Option<Read<'s, LevelStart>>,
        ReadExpect<'s, Level>,
        WriteStorage<'s, Handle<Prefab<ThrownPrefab>>>,
        Read<'s, ThrownPrefabSet>,
    );

    fn run(
        &mut self,
        (
            entities,
            storage,
            sounds,
            audio_output,
            level_start,
            level,
            prefabs,
            throwable_prefab_set,
        ): Self::SystemData,
    ) {
        // Only run once we have an initialised level start
        if let Some(level_start) = level_start {
            let new_beat =
                (level_start.0.elapsed().as_secs_f64() / SECONDS_PER_BEAT).floor() as u64;

            // If new beat changes, update beat and act
            if new_beat != self.beat {
                info!("BEAT {}", new_beat);
                self.beat = new_beat;
                self.beat_act(
                    storage,
                    sounds,
                    audio_output,
                    level,
                    entities,
                    prefabs,
                    throwable_prefab_set,
                );
            }
        }
    }
}

impl TimingSystem {
    #[allow(clippy::too_many_arguments)]
    fn beat_act<'s>(
        &mut self,
        storage: Read<'s, AssetStorage<Source>>,
        sounds: ReadExpect<'s, audio::Sounds>,
        audio_output: Option<Read<'s, Output>>,
        level: ReadExpect<'s, Level>,
        entities: Entities<'s>,
        mut prefabs: WriteStorage<'s, Handle<Prefab<ThrownPrefab>>>,
        throwable_prefab_set: Read<'s, ThrownPrefabSet>,
    ) {
        // Iterate through all events in current beat
        if let Some(events) = level.events.get(&self.beat) {
            for event in events {
                info!("Actioning event {:?}", event);
                match event {
                    GameEvent::ThrowStart { t: _ } => {
                        audio::play_throw(&*sounds, &storage, audio_output.as_deref());
                    }
                    GameEvent::ThrowAnimationStart => {
                        // Start throw animation
                    }
                    GameEvent::ThrowObject { t } => {
                        let throwable_prefab = throwable_prefab_set
                            .get_handle(*t)
                            .expect("Throwable prefab set get_handle() failed");
                        entities
                            .build_entity()
                            .with(throwable_prefab, &mut prefabs)
                            .build();
                    }
                    GameEvent::ThrowEnd => {
                        // Reset thrower animation
                    }
                }
            }
        }
    }
}
