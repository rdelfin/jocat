use crate::animation::AnimationId;
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    prelude::{Builder, World, WorldExt},
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
}

pub fn load_player(world: &mut World, progress_counter: &mut ProgressCounter) {
    let prefab = world.exec(|loader: PrefabLoader<'_, PlayerPrefab>| {
        loader.load("prefab/player.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(prefab).build();
}
