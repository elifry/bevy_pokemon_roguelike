use bevy::prelude::*;

use self::assets::DataAssetsPlugin;

pub mod assets;

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
      app.add_plugins(DataAssetsPlugin);
    }
}
