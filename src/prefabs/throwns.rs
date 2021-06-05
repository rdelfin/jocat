use crate::components::FallingObject;
use amethyst::{
    assets::{Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::World,
    renderer::sprite::prefab::SpriteScenePrefab,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct ThrownPrefab {
    sprite_scene: SpriteScenePrefab,
    falling_object: FallingObject,
}

#[derive(Default)]
pub struct ThrownPrefabSet(pub Vec<Handle<Prefab<ThrownPrefab>>>);

pub fn load_thrown(world: &mut World, progress_counter: &mut ProgressCounter) {
    let prefab = world.exec(|loader: PrefabLoader<'_, ThrownPrefab>| {
        loader.load("prefabs/throwable_log.ron", RonFormat, progress_counter)
    });
    world.insert(ThrownPrefabSet(vec![prefab]));
}
