use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    renderer::sprite::prefab::SpriteScenePrefab,
};
use serde::{Deserialize, Serialize};

use crate::component;

#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
pub struct Creature {
    sprite_scene: SpriteScenePrefab,
    movable: Option<component::Movable>,
    player: Option<component::Player>,
}
