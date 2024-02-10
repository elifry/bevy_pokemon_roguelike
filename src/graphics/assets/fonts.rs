use bevy::prelude::*;

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAsset>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct FontAsset(pub Handle<Font>);
