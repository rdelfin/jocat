use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    Watermelon,
    Wheat,
    Log,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum GameEvent {
    ThrowStart { t: ObjectType },
    // Auto-populated, do not add
    ThrowAnimationStart,
    // Auto-populated, do not add
    ThrowObject { t: ObjectType },
    // Auto-populated, do not add
    ThrowEnd,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub events: BTreeMap<u64, HashSet<GameEvent>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected event {0:?}. Please don't use auto-populated events")]
    UnexpectedEvent(GameEvent),
}

impl Level {
    /// Many events are inferred and should not be manually added. This function fills
    /// them in.
    pub fn populate(&mut self) -> Result<(), Error> {
        self.merge_events(self.get_generated_events()?);

        Ok(())
    }

    fn get_generated_events(&self) -> Result<BTreeMap<u64, HashSet<GameEvent>>, Error> {
        let mut new_events = BTreeMap::new();
        // Iterate over every event
        for (beat, beat_events) in self.events.iter() {
            for event in beat_events {
                match event {
                    GameEvent::ThrowStart { t } => {
                        let anim_start_beat = beat + 1;
                        let throw_obj_beat = beat + 2;
                        let throw_end_beat = beat + 3;

                        add_event_to(
                            &mut new_events,
                            GameEvent::ThrowAnimationStart,
                            anim_start_beat,
                        );
                        add_event_to(
                            &mut new_events,
                            GameEvent::ThrowObject { t: *t },
                            throw_obj_beat,
                        );
                        add_event_to(&mut new_events, GameEvent::ThrowEnd, throw_end_beat);
                    }
                    _ => return Err(Error::UnexpectedEvent(event.clone())),
                }
            }
        }
        Ok(new_events)
    }

    fn merge_events(&mut self, new_events: BTreeMap<u64, HashSet<GameEvent>>) {
        for (beat, beat_events) in new_events {
            let old_events = self.events.entry(beat).or_insert_with(HashSet::new);
            // Alternatively, this could be an iterate + insert (more efficient)
            *old_events = old_events.union(&beat_events).cloned().collect();
        }
    }
}

fn add_event_to(events: &mut BTreeMap<u64, HashSet<GameEvent>>, new_event: GameEvent, beat: u64) {
    events
        .entry(beat)
        .or_insert_with(HashSet::new)
        .insert(new_event);
}
