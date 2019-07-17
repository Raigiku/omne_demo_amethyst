use amethyst::{
    assets::{PrefabData},
    derive::PrefabData,
    ecs::{Entity, WriteStorage, Component, DenseVecStorage},
    Error,
};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

#[derive(Clone, Copy, Component, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Position{
    x: f32,
    y: f32,
    z: f32
}
