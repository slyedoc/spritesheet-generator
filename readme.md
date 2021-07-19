# spritesheet-generator

Started from [spritesheet-generator](https://github.com/rafaeldelboni/spritesheet-generator)

I wanted an easy to use cli tool for generating spritessheets for bevy, with a lib to export format info for deserialize
to bevy

This is really just exposing [texture_packer](https://github.com/PistonDevelopers/texture_packer).

Added built in cli for generating and then a lib that just export spritesheet for deserialize in bevy

## Notes

When using with cargo you can pass commandline arguments with the '--'
```cargo run -- --help```

Examples

```bash
cargo run --bin sprite-gen --features build-binary -- -i ../kenney/stone-floor -o ../kenney/generated -n stone-floor --format ron
```
