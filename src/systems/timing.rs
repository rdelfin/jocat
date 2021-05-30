use crate::{
    audio,
    resources::{GameEventType, Level, LevelStart},
};
use amethyst::{
    assets::AssetStorage,
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
    type SystemData = (
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, audio::Sounds>,
        Option<Read<'s, Output>>,
        Option<Read<'s, LevelStart>>,
        ReadExpect<'s, Level>,
    );

    fn run(&mut self, (storage, sounds, audio_output, level_start, level): Self::SystemData) {
        // Only run once we have an initialised level start
        if let Some(level_start) = level_start {
            let new_beat =
                (level_start.0.elapsed().as_secs_f64() / SECONDS_PER_BEAT).floor() as u64;

            // If new beat changes, update beat and act
            if new_beat != self.beat {
                info!("BEAT {}", new_beat);
                self.beat = new_beat;
                self.beat_act(storage, sounds, audio_output, level);
            }
        }
    }
}

impl TimingSystem {
    fn beat_act<'s>(
        &mut self,
        storage: Read<'s, AssetStorage<Source>>,
        sounds: ReadExpect<'s, audio::Sounds>,
        audio_output: Option<Read<'s, Output>>,
        level: ReadExpect<'s, Level>,
    ) {
        if level.events.contains_key(&self.beat) {
            match level.events[&self.beat].e {
                GameEventType::ThrowSmall => {
                    audio::play_throw(&*sounds, &storage, audio_output.as_deref());
                }
            }
        }
    }
}
