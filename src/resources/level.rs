use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameEventType {
    ThrowSmall,
    Attack,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    // What event happens at this beat?
    pub e: GameEventType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub events: BTreeMap<u64, GameEvent>,
}
