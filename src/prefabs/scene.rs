use amethyst::{
    assets::{Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::{Builder, World, WorldExt},
    renderer::sprite::prefab::SpriteScenePrefab,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct BackgroundPrefab {
    sprite_scene: SpriteScenePrefab,
}

#[derive(Default)]
pub struct ThrowablePrefabSet(pub Vec<Handle<Prefab<BackgroundPrefab>>>);

pub fn load_background(world: &mut World, progress_counter: &mut ProgressCounter) {
    let prefab = world.exec(|loader: PrefabLoader<'_, BackgroundPrefab>| {
        loader.load("prefabs/background.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(prefab).build();
}

pub fn load_throwable(world: &mut World, progress_counter: &mut ProgressCounter) {
    let prefab = world.exec(|loader: PrefabLoader<'_, BackgroundPrefab>| {
        loader.load("prefabs/throwable_log.ron", RonFormat, progress_counter)
    });
    world.insert(ThrowablePrefabSet(vec![prefab]));
}
