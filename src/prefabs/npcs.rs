use crate::{animation::AnimationId, components::Npc};
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

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct NpcPrefab {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
    npc: Npc,
}

pub fn load_jocrap(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<NpcPrefab>> {
    load_npc_prefab(world, "prefabs/jocrap.ron", progress_counter)
}

fn load_npc_prefab(
    world: &mut World,
    prefab_location: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<NpcPrefab>> {
    world.exec(|loader: PrefabLoader<'_, NpcPrefab>| {
        loader.load(prefab_location, RonFormat, progress_counter)
    })
}
