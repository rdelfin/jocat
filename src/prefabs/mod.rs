mod npcs;
mod player;
mod scene;
mod throwns;

pub use self::{
    npcs::{load_jocrap, NpcPrefab},
    player::{load_player, PlayerPrefab},
    scene::{load_background, BackgroundPrefab},
    throwns::ThrownPrefab,
};
