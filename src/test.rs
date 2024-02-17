use bevy::prelude::*;
use bitmap_font::fonts::BitmapFont;

use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::sprite_text::{
    SpriteText, SpriteTextBundle, SpriteTextStyle, Text2DSpriteBundle,
};
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test);
    }
}

fn spawn_test(font_assets: Res<FontAssets>, mut commands: Commands) {
    let text_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        ..default()
    };
    commands
        .spawn(Text2DSpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
            text_anchor: bevy::sprite::Anchor::BottomLeft,
            text: SpriteText::from_section("Hello world!", text_style.clone()),
            ..default()
        })
        .insert(Name::new("TextSprite Test"));

    // let box_size = Vec2::new(150.0, 80.0);
    // let box_position = Vec2::new(300., 35.0);
    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             custom_size: Some(Vec2::new(box_size.x, box_size.y)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(box_position.extend(20.)),
    //         ..default()
    //     })
    //     .insert(Name::new("Boxed SpriteText"))
    //     // .with_children(|builder| {
    //     //     builder.spawn(Text2dBundle {
    //     //         text: Text {
    //     //             sections: vec![TextSection::new(
    //     //                 "this text wraps in the box\n(Unicode linebreaks)",
    //     //                 TextStyle::default(),
    //     //             )],
    //     //             alignment: TextAlignment::Left,
    //     //             linebreak_behavior: BreakLineOn::WordBoundary,
    //     //         },
    //     //         text_2d_bounds: Text2dBounds {
    //     //             // Wrap text in the rectangle
    //     //             size: box_size,
    //     //         },
    //     //         // ensure the text is drawn on top of the box
    //     //         transform: Transform::from_translation(Vec3::Z),
    //     //         ..default()
    //     //     });
    //     // });
    //     .with_children(|builder| {
    //         builder.spawn(Text2DSpriteBundle {
    //             // text_anchor: bevy::sprite::Anchor::TopLeft,
    //             text: SpriteText {
    //                 sections: vec![
    //                     SpriteTextSection::new(
    //                         "this text wraps in the box (Unicode linebreaks) \nthis text wraps in the box (Unicode linebreaks)",
    //                         text_style.clone(),
    //                     ),
    //                     SpriteTextSection::new(
    //                         " Another text section",
    //                         text_style.clone(),
    //                     ),
    //                 ],
    //                 alignment: TextAlignment::Center,
    //                 linebreak_behavior: BreakLineOn::WordBoundary,
    //                 ..default()
    //             },
    //             text_2d_bounds: Text2dBounds {
    //                 // Wrap text in the rectangle
    //                 size: box_size,
    //             },
    //             // ensure the text is drawn on top of the box
    //             transform: Transform::from_translation(Vec3::Z),
    //             ..default()
    //         });
    //     });

    // Text with one section
    // ImageBundle
    // UI test
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             justify_content: JustifyContent::SpaceBetween,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Name::new("Node Text"),
    //             // Create a TextBundle that has a Text with a single section.
    //             SpriteTextBundle {
    //                 text: SpriteText::from_section("hello hello hello bevy!", text_style.clone()),
    //                 style: Style {
    //                     margin: UiRect::all(Val::VMin(3.)),
    //                     align_self: AlignSelf::FlexStart,
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //         ));

    //         // parent.spawn(ImageBundle {
    //         //     image: UiImage::new(asset_server.load("test.png")),
    //         //     background_color: Color::WHITE.into(),
    //         //     ..default()
    //         // });

    //         // parent.spawn((
    //         //     NodeBundle {
    //         //         style: Style {
    //         //             width: Val::Px(16.0),
    //         //             height: Val::Px(17.0),
    //         //             left: Val::Px(100.),
    //         //             margin: UiRect::top(Val::VMin(5.)),
    //         //             ..default()
    //         //         },
    //         //         // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
    //         //         background_color: Color::WHITE.into(),
    //         //         ..default()
    //         //     },
    //         //     UiImage::new(asset_server.load("test.png")),
    //         // ));

    //         // Set the alignment of the Text
    //         // .with_text_alignment(TextAlignment::Center)
    //         // // Set the style of the TextBundle itself.
    //         // .with_style(Style {
    //         //     position_type: PositionType::Absolute,
    //         //     bottom: Val::Px(5.0),
    //         //     right: Val::Px(5.0),
    //         //     ..default()

    //         // parent.spawn((
    //         //     ImageBundle {
    //         //         image: UiImage::new(asset_server.load("logo.png")),
    //         //         ..default()
    //         //     },
    //         //     // NodeBundle {
    //         //     //     style: Style {
    //         //     //         // width: Val::Px(500.0),
    //         //     //         // height: Val::Px(125.0),
    //         //     //         margin: UiRect::top(Val::VMin(5.)),
    //         //     //         ..default()
    //         //     //     },
    //         //     //     // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
    //         //     //     background_color: Color::WHITE.into(),
    //         //     //     ..default()
    //         //     // },
    //         //     // UiImage::new(asset_server.load("logo.png")),
    //         // ));
    //     });
}
