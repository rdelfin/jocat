use crate::{prefabs::ThrownPrefab, resources::ObjectType};
use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    prelude::World,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct ThrownPrefabSet {
    prefab_handles: HashMap<ObjectType, Handle<Prefab<ThrownPrefab>>>,
}

impl ThrownPrefabSet {
    pub fn new(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        // This iterator is only here to ensure that any time new item types are added, the match
        // will cause a compiler error, and this array should get caught more easily
        let prefab_handles = [ObjectType::Log, ObjectType::Watermelon, ObjectType::Wheat]
            .iter()
            .map(|obj_type| {
                (
                    *obj_type,
                    // Remember to also update the array above!
                    match obj_type {
                        ObjectType::Log => {
                            load_thrown_prefab("prefabs/throwable_log.ron", world, progress_counter)
                        }
                        ObjectType::Wheat => load_thrown_prefab(
                            "prefabs/throwable_wheat.ron",
                            world,
                            progress_counter,
                        ),
                        ObjectType::Watermelon => load_thrown_prefab(
                            "prefabs/throwable_watermelon.ron",
                            world,
                            progress_counter,
                        ),
                    },
                )
            })
            .collect();

        ThrownPrefabSet { prefab_handles }
    }

    pub fn get_handle(&self, obj_type: ObjectType) -> anyhow::Result<Handle<Prefab<ThrownPrefab>>> {
        Ok(self
            .prefab_handles
            .get(&obj_type)
            .ok_or_else(|| {
                anyhow::anyhow!("Prefab for object type {:?} was not loaded.", obj_type)
            })?
            .clone())
    }
}

fn load_thrown_prefab(
    prefab_path: &str,
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<ThrownPrefab>> {
    world.exec(|loader: PrefabLoader<'_, ThrownPrefab>| {
        loader.load(prefab_path, RonFormat, progress_counter)
    })
}
