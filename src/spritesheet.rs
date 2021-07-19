use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Screen {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub screen: Screen,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spritesheet {
    pub frames: Vec<Frame>,
}
