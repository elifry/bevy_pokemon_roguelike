pub mod pokemon_data;
pub mod spell_data;
pub mod text_data;

use self::pokemon_data::PokemonDataPlugin;
use self::spell_data::SpellDataPlugin;
use self::text_data::TextDataPlugin;
use bevy::prelude::*;

pub struct DataAssetsPlugin;

impl Plugin for DataAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PokemonDataPlugin, SpellDataPlugin, TextDataPlugin));
    }
}
