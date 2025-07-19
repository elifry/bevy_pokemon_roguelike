use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_pokemon_roguelike::GamePlugin;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1.,
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Unprocessed,
                    meta_check: AssetMetaCheck::Always,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy pokemon roguelike".to_string(), // ToDo
                        // Bind to canvas included in `index.html`
                        canvas: Some("#game".to_owned()),
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(GamePlugin)
        .run();
}
