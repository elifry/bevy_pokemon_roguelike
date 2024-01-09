use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_pokemon_roguelike::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1.,
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // mode: AssetMode::Processed,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy pokemon roguelike".to_string(), // ToDo
                        // Bind to canvas included in `index.html`
                        canvas: Some("#game".to_owned()),
                        // The canvas size is constrained in index.html and build/web/styles.css
                        fit_canvas_to_parent: true,
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
