
use texture_packer::exporter::ImageExporter;
use texture_packer::texture::Texture;
use texture_packer::{TexturePacker, TexturePackerConfig};

use spritesheet_generator::{config, file_texture, spritesheet};
use std::fs::File;
use clap::{Clap};

fn main() {
    let config = config::Config::parse();

    // Perform texture packing
    let tp_config = TexturePackerConfig {
        max_width: config.max_width,
        max_height: config.max_height,
        border_padding: config.border_padding,
        allow_rotation: config.allow_rotation,
        texture_outlines: config.debug_outlines,
        ..Default::default()
    };
    let mut packer = TexturePacker::new_skyline(tp_config);
    for file_textures in file_texture::find_all(config.input_folder) {
        packer.pack_own(file_textures.file.name, file_textures.texture)
            .expect("Error adding file");
    }

    // Save
    let atlas = spritesheet::to_atlas(packer.get_frames(), packer.width(), packer.height());
    match config.format {
        spritesheet_generator::config::Format::Json => {
            let json_path = format!("{}/{}.json", config.output_folder, config.name);
            let json_file = File::create(json_path).unwrap();
            serde_json::to_writer_pretty(json_file, &atlas).unwrap();
        },
        spritesheet_generator::config::Format::Ron => {
            let ron_path = format!("{}/{}.ron", config.output_folder, config.name);
            let ron_file = File::create(ron_path).unwrap();
            let pretty = ron::ser::PrettyConfig::new()
                .with_separate_tuple_members(true)
                .with_enumerate_arrays(true);
            ron::ser::to_writer_pretty(ron_file, &atlas, pretty).unwrap();
        },
    }

    // Save Image
    let exporter = ImageExporter::export(&packer).unwrap();
    let image_path = format!("{}/{}.png", config.output_folder, config.name);
    let mut image_file = File::create(image_path).unwrap();
    exporter.write_to(&mut image_file, image::ImageFormat::Png).unwrap();
}
