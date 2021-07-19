use serde::{Deserialize, Serialize};
use bevy::reflect::TypeUuid;

#[derive(TypeUuid)]
#[uuid = "15c92900-00a3-44cf-ba48-c7518d9b3adf"]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteSheet {
    pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
