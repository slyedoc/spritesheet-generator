use serde::{Deserialize, Serialize};
use bevy::reflect::TypeUuid;

#[derive(TypeUuid)]
#[uuid = "15c92900-00a3-44cf-ba48-c7518d9b3adf"]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteSheet {
    pub width: u32,
    pub height: u32,
    pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub name: String,
    pub position: Rect,
    pub trimmed: bool,
    pub orginal: Rect,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rect {
    /// Horizontal position the rectangle begins at.
    pub x: u32,
    /// Vertical position the rectangle begins at.
    pub y: u32,
    /// Width of the rectangle.
    pub w: u32,
    /// Height of the rectangle.
    pub h: u32,
}
