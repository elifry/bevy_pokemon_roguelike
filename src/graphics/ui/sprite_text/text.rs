use bevy::{
    prelude::{default, *},
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};
use bitmap_font::fonts::BitmapFont;

#[derive(Bundle, Default, Clone, Debug)]
pub struct Text2DSpriteBundle {
    /// Contains the text.
    pub text: SpriteText,
    /// How the text is positioned relative to its transform.
    pub text_anchor: Anchor,
    /// The maximum width and height of the text.
    pub text_2d_bounds: Text2dBounds,
    /// The transform of the text.
    pub transform: Transform,
    /// The global transform of the text.
    pub global_transform: GlobalTransform,
    /// The visibility properties of the text.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    // Internal rendering
    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct SpriteText {
    pub sections: Vec<SpriteTextSection>,
    /// The text's internal alignment.
    /// Should not affect its position within a container.
    pub alignment: TextAlignment,
    /// How the text should linebreak when running out of the bounds determined by max_size
    pub linebreak_behavior: BreakLineOn,
}

#[derive(Debug, Clone, Reflect)]
pub struct SpriteTextStyle {
    pub font: Handle<BitmapFont>,
    pub font_size: f32,
    pub color: Color,
    pub background_color: Option<Color>,
}

impl Default for SpriteTextStyle {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_size: Default::default(),
            color: Color::WHITE,
            background_color: Default::default(),
        }
    }
}

impl SpriteText {
    pub fn from_section(value: impl Into<String>, style: SpriteTextStyle) -> Self {
        Self {
            sections: vec![SpriteTextSection::new(value, style)],
            ..default()
        }
    }

    pub fn from_sections(sections: impl IntoIterator<Item = SpriteTextSection>) -> Self {
        Self {
            sections: sections.into_iter().collect(),
            ..default()
        }
    }

    /// Returns this [`Text`] with a new [`TextAlignment`].
    pub const fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Returns this [`Text`] with soft wrapping disabled.
    /// Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, will still occur.
    pub const fn with_no_wrap(mut self) -> Self {
        self.linebreak_behavior = BreakLineOn::NoWrap;
        self
    }

    /// Returns the total number of chars in all [`SpriteTextSection`]
    pub fn total_chars_count(&self) -> usize {
        self.sections
            .iter()
            .fold(0, |sum, section| sum + section.value.chars().count())
    }
}

impl Default for SpriteText {
    fn default() -> Self {
        Self {
            sections: Default::default(),
            alignment: TextAlignment::Left,
            linebreak_behavior: BreakLineOn::WordBoundary,
        }
    }
}

#[derive(Debug, Default, Clone, Reflect)]
pub struct SpriteTextSection {
    pub value: String,
    pub style: SpriteTextStyle,
}

impl SpriteTextSection {
    /// Create a new [`SpriteTextSection`].
    pub fn new(value: impl Into<String>, style: SpriteTextStyle) -> Self {
        Self {
            value: value.into(),
            style,
        }
    }

    /// Create an empty [`SpriteTextSection`] from a style. Useful when the value will be set dynamically.
    pub const fn from_style(style: SpriteTextStyle) -> Self {
        Self {
            value: String::new(),
            style,
        }
    }
}
