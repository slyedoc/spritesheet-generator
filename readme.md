# spritesheet_generator

Started from [spritesheet-generator](https://github.com/rafaeldelboni/spritesheet-generator), I might reloadup this without the fork later since its so different, but wanted to give credit.

I wanted an easy to use cli tool for generating spritessheets for bevy, with a lib to export format info for deserialize.

And without having workspaces, to try it out.

This is really just exposing [texture_packer](https://github.com/PistonDevelopers/texture_packer).

## Usages

```bash

USAGE:
    sprite-gen [FLAGS] [OPTIONS] --input-folder <input-folder>

FLAGS:
    -a, --allow-rotation
    -d, --debug-outlines
        --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -b, --border-padding <border-padding>    [default: 2]
    -f, --format <format>                    [default: ron] [possible values: json, ron]
    -h, --height <height>                    [default: 1024]
    -i, --input-folder <input-folder>
    -n, --name <name>                        [default: filename]
    -o, --output-folder <output-folder>      [default: ./]
    -w, --width <width>                      [default: 1024]

```

## My Usages

```bash
cargo run --bin sprite-gen --features build-binary -- -i ../kenney/dungeon_pack/isometric -o ../kenney/generated -n dungeon_pack_iso -h 4096 -w 4096
```

```rust
use spritesheet_generator::sprite_sheet::SpriteSheet;

/// Used to import our own custom TextureAtlas from sprite sheet generator
pub fn from_sprite_sheet(texture: Handle<Texture>, sprite_sheet: &SpriteSheet) -> TextureAtlas {
    let mut sprites = Vec::new();
    for f in sprite_sheet.frames.iter() {
        let rec = sprite::Rect {
            min: Vec2::new(f.position.x as f32, f.position.y as f32),
            max: Vec2::new(
                (f.position.x + f.position.w) as f32,
                (f.position.y + f.position.h) as f32,
            ),
        };
        sprites.push(rec)
    }

    TextureAtlas {
        size: Vec2::new(sprite_sheet.width as f32, sprite_sheet.height as f32),
        textures: sprites,
        texture,
        texture_handles: None,
    }
}
```

## Notes

When using with cargo you can pass commandline arguments with the '--'
```cargo run -- --help```

Got Uuid from [uuidtools](https://www.uuidtools.com/generate/v4)