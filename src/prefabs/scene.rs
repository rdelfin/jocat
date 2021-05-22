use amethyst::{
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
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

pub fn load_background(world: &mut World, progress_counter: &mut ProgressCounter) {
    let prefab = world.exec(|loader: PrefabLoader<'_, BackgroundPrefab>| {
        loader.load("prefabs/background.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(prefab).build();
}
