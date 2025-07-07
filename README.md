# Bevy Pokemon Roguelike

This project aims to recreate a game similar to _Pokémon Mystery Dungeon: Red Rescue Team_ by replicating the gameplay mechanics and graphics using [Bevy](https://bevyengine.org/).

![Demo Bevy Pokemon Roguelike](./docs/demo.png)

---

## Development

### Prerequisites

Before starting the project, ensure you have the Rust toolchain installed. You can install it from [rustup.rs](https://rustup.rs/).

### Generating Assets

The game assets must be compiled before running the game. Use the following command to build them:

```sh
cargo run --bin=assets_builder --package=assets_builder
```

## Running the Game

Once the assets are created, you can run the game with the following command:

```sh
cargo run  --bin=bevy_pokemon_roguelike --package=bevy_pokemon_roguelike
```

## Structure

### crates/assets_builder

This package is responsible for preparing game assets from the source files in a way that is optimized for performance and ease of use by the game engine. Key responsibilities include:

- Building a font atlas for the bitmap font.
- Converting Pokémon data from JSON format into RON files.
- Creating {pokemon-name}.chara binary files containing animation data, including frame animation images and associated metadata.

### crates/bitmap_font

TODO

## TODO

- Use the stat in the pokemon data to set up the Health / Stats components
- add Text component to draw text according to a font take inspiration there
  <https://github.com/StaffEngineer/bevy_cosmic_edit/blob/a44020ac517c34d381c72563e848a8f8ebde96c6/src/render.rs#L186>
