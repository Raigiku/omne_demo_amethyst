use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{prelude::Entity},
    error::Error,
    renderer::{sprite::{prefab::SpriteScenePrefab}}
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
pub struct Human {
    sprite_scene: SpriteScenePrefab
}
