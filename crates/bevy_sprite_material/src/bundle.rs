use crate::{SpriteMaterial, TextureAtlasSpriteMaterial};
use bevy_asset::Handle;
use bevy_ecs::bundle::Bundle;
use bevy_render::view::{InheritedVisibility, ViewVisibility, Visibility};
use bevy_sprite::{ColorMaterial, TextureAtlas};
use bevy_transform::components::{GlobalTransform, Transform};

/// Component bundle for 2D sprites with a `ColorMaterial`
#[derive(Bundle, Clone, Default)]
pub struct SpriteMaterialBundle {
    /// The main sprite component
    pub sprite: SpriteMaterial,
    /// transform component, defining translation/rotation/scale informations
    pub transform: Transform,
    /// transform component, defining translation/rotation/scale informations
    pub global_transform: GlobalTransform,
    /// The sprite material, defining its texture and color
    pub material: Handle<ColorMaterial>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
}

/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`) with a `ColorMaterial`
#[derive(Bundle, Clone, Default)]
pub struct SpriteMaterialSheetBundle {
    /// The specific sprite from the texture atlas to be drawn, defaulting to the sprite at index 0.
    pub sprite: TextureAtlasSpriteMaterial,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub transform: Transform,
    /// transform component, defining translation/rotation/scale informations
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// User indication of whether an entity is visible from its parent
    pub inherited_visibility: InheritedVisibility,
    /// The sprite material, defining its texture and color
    pub material: Handle<ColorMaterial>,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
