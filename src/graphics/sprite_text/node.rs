use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
    ui::ContentSize,
};

use super::{SpriteText, SpriteTextSection, SpriteTextStyle};

#[derive(Bundle, Debug)]
pub struct SpriteTextBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// Contains the text of the node
    pub text: SpriteText,
    /// The calculated size based on the given image
    pub calculated_size: ContentSize,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `TextBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,

    // Internal rendering
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub text_2d_bounds: Text2dBounds,
    pub text_anchor: Anchor,
}

impl Default for SpriteTextBundle {
    fn default() -> Self {
        Self {
            text: Default::default(),
            calculated_size: Default::default(),
            node: Default::default(),
            style: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
            // Transparent background
            background_color: BackgroundColor(Color::NONE),
            sprite: Default::default(),
            texture: Default::default(),
            text_2d_bounds: Default::default(),
            text_anchor: Default::default(),
        }
    }
}

impl SpriteTextBundle {
    /// Create a [`SpriteTextBundle`] from a single section.
    ///
    /// See [`Text::from_section`] for usage.
    pub fn from_section(value: impl Into<String>, style: SpriteTextStyle) -> Self {
        Self {
            text: SpriteText::from_section(value, style),
            ..Default::default()
        }
    }

    /// Create a [`SpriteTextBundle`] from a list of sections.
    ///
    /// See [`Text::from_sections`] for usage.
    pub fn from_sections(sections: impl IntoIterator<Item = SpriteTextSection>) -> Self {
        Self {
            text: SpriteText::from_sections(sections),
            ..Default::default()
        }
    }

    /// Returns this [`SpriteTextBundle`] with a new [`TextAlignment`] on [`Text`].
    pub const fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.text.alignment = alignment;
        self
    }

    /// Returns this [`SpriteTextBundle`] with a new [`Style`].
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Returns this [`SpriteTextBundle`] with a new [`BackgroundColor`].
    pub const fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = BackgroundColor(color);
        self
    }

    /// Returns this [`SpriteTextBundle`] with soft wrapping disabled.
    /// Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, will still occur.
    pub const fn with_no_wrap(mut self) -> Self {
        self.text.linebreak_behavior = BreakLineOn::NoWrap;
        self
    }
}
