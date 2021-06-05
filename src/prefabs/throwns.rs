use crate::components::FallingObject;
use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    renderer::sprite::prefab::SpriteScenePrefab,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct ThrownPrefab {
    sprite_scene: SpriteScenePrefab,
    falling_object: FallingObject,
}
