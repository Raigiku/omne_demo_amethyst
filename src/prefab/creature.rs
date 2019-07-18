use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{prelude::Entity},
    error::Error,
    renderer::{sprite::{prefab::SpriteScenePrefab}}
};
use serde::{Serialize, Deserialize};

use crate::component::{PlayerComponent, MovableComponent};

#[derive(Debug, Clone, Serialize, Deserialize, PrefabData)]
pub struct CreaturePrefab {
    sprite_scene: SpriteScenePrefab,
    movable: Option<MovableComponent>,
    player: Option<PlayerComponent>
}
