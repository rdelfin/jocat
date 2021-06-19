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
    load_player_prefab(world, "prefabs/player.ron", progress_counter)
}

pub fn load_jocrap(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<PlayerPrefab>> {
    load_player_prefab(world, "prefabs/jocrap.ron", progress_counter)
}

fn load_player_prefab(
    world: &mut World,
    prefab_location: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<PlayerPrefab>> {
    world.exec(|loader: PrefabLoader<'_, PlayerPrefab>| {
        loader.load(prefab_location, RonFormat, progress_counter)
    })
    // world.create_entity().with(prefab).build();
}
