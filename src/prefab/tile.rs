use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    renderer::sprite::prefab::SpriteScenePrefab,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
pub struct Tile {
    sprite_scene: SpriteScenePrefab,
}
