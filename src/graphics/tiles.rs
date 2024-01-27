use bevy::prelude::*;

use crate::{
    map::{Position, Tile},
    GameState,
};

use super::{assets::TileAssets, TILE_SIZE, TILE_Z};

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_tile_renderer.run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<TileAssets>,
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(13);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        //sprite.color = Color::OLIVE;
        let v = super::get_world_position(&position.0, TILE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.forest_path.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
