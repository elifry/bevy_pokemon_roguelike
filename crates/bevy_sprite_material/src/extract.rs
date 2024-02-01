use crate::{SpriteMaterial, TextureAtlasSpriteMaterial};
use bevy_asset::{Assets, Handle};
use bevy_ecs::{
    entity::Entity,
    prelude::{Query, Res, ResMut},
};
use bevy_render::{prelude::Visibility, Extract, MainWorld};
use bevy_sprite::{ColorMaterial, ExtractedSprite, ExtractedSprites, TextureAtlas};
use bevy_transform::prelude::GlobalTransform;

pub fn extract_sprites(
    //mut render_world: ResMut<MainWorld>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    materials: Extract<Res<Assets<ColorMaterial>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    sprite_query: Extract<
        Query<(
            Entity,
            &Visibility,
            &SpriteMaterial,
            &GlobalTransform,
            &Handle<ColorMaterial>,
        )>,
    >,
    atlas_query: Extract<
        Query<(
            Entity,
            &Visibility,
            &TextureAtlasSpriteMaterial,
            &GlobalTransform,
            &Handle<ColorMaterial>,
        )>,
    >,
) {
    println!("extract_sprites");
    // Regular Sprites
    for (entity, visibility, sprite, transform, handle) in sprite_query.iter() {
        // if visibility != Visibility::Visible {
        //     continue;
        // }
        let material = materials.get(handle).cloned().unwrap_or_default();
        let (color, image_handle_id) = (material.color, material.texture.unwrap().id());
        // PERF: we don't check in this function that the `Image` asset is ready, since it should be in most cases and hashing the handle is expensive
        extracted_sprites.sprites.insert(
            entity,
            ExtractedSprite {
                color,
                transform: *transform,
                // Use the full texture
                rect: None,
                // Pass the custom size
                custom_size: sprite.custom_size,
                flip_x: sprite.flip_x,
                flip_y: sprite.flip_y,
                image_handle_id,
                anchor: sprite.anchor.as_vec(),
                original_entity: Some(entity), // Maybe use the entity there
            },
        );
    }
    // Atlas Sprites
    for (entity, visibility, atlas_sprite, transform, handle) in atlas_query.iter() {
        // if visibility != Visibility::Visible {
        //     continue;
        // }
        let material = materials.get(handle).cloned().unwrap_or_default();
        let (color, image_handle_id) = (material.color, material.texture.unwrap().id());
        if let Some(texture_atlas) = texture_atlases.get(image_handle_id.untyped()) {
            let rect = Some(texture_atlas.textures[atlas_sprite.index as usize]);
            extracted_sprites.sprites.insert(
                entity,
                ExtractedSprite {
                    color,
                    transform: *transform,
                    // Select the area in the texture atlas
                    rect,
                    // Pass the custom size
                    custom_size: atlas_sprite.custom_size,
                    flip_x: atlas_sprite.flip_x,
                    flip_y: atlas_sprite.flip_y,
                    image_handle_id,
                    anchor: atlas_sprite.anchor.as_vec(),
                    original_entity: Some(entity), // Maybe use the entity there
                },
            );
        }
    }
}
