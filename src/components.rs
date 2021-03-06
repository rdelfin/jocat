use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
    Error,
};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Player {
    dummy: bool,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct FallingObject {
    pub g: f32,
    pub v: Vector2<f32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum NpcType {
    JoCrap,
    Gobo,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Npc {
    pub t: NpcType,
}
