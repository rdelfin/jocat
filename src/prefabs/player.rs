use crate::{animation::AnimationId, components::Player};
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::World,
    renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender},
};
use serde::Deserialize;

/// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct PlayerPrefab {
    /// Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    /// Аll animations that can be run on the entity
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
    player: Option<Player>,
}

pub fn load_player(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<PlayerPrefab>> {
    world.exec(|loader: PrefabLoader<'_, PlayerPrefab>| {
        loader.load("prefabs/player.ron", RonFormat, progress_counter)
    })
}
