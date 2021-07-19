use serde::{Deserialize, Serialize};

#[cfg(feature = "build-binary")]
use texture_packer;

#[cfg(feature = "build-binary")]
use std::collections::HashMap;

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

#[cfg(feature = "build-binary")]
pub fn to_atlas<K>(
    frames: &HashMap<String, texture_packer::Frame<K>>,
    image_width: u32,
    image_height: u32,
) -> Spritesheet {

    let frames_map = frames
        .iter()
        .map(|(name, frame)| Frame {
            name: name.to_owned(),
            x: frame.frame.x,
            y: frame.frame.y,
            w: frame.frame.w,
            h: frame.frame.h,
            screen: Screen {
                x: 1. / (image_width as f32 / frame.frame.x as f32),
                y: 1. / (image_height as f32 / frame.frame.y as f32),
                w: 1. / (image_width as f32 / frame.frame.w as f32),
                h: 1. / (image_height as f32 / frame.frame.h as f32),
            },
        })
        .collect();

    Spritesheet { frames: frames_map }
}
