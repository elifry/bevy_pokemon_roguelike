use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_inspector_egui::bevy_egui::egui::Color32;
use bevy_inspector_egui::bevy_egui::{egui, EguiContexts};

use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::assets::ui_assets::UIAssets;
use crate::graphics::ui::sprite_text::{
    SpriteText, SpriteTextSection, SpriteTextStyle, Text2DSpriteBundle,
};
use crate::graphics::ui::{BorderedFrame, SpriteTextEguiUiExt, UISpriteText, UISpriteTextSection};
use crate::graphics::world_number::{WorldNumber, WorldNumberType};
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test);
        //.add_systems(Update, ui.run_if(in_state(GameState::Playing)));
    }
}

fn ui(mut ctx: EguiContexts, font_assets: Res<FontAssets>, ui_assets: Res<UIAssets>) {
    let ctx = ctx.ctx_mut();

    egui::TopBottomPanel::bottom("bottom")
        .frame(egui::Frame::none())
        .show_separator_line(false)
        .exact_height(64.)
        .show(ctx, |ui| {
            let outer_margin = UiRect::all(Val::Px(8.));

            BorderedFrame::new(&ui_assets.panel_blue)
                .background(&ui_assets.transparent_panel_bg)
                .padding(UiRect::axes(Val::Px(12.), Val::Px(10.)))
                .margin(outer_margin)
                .show(ui, |ui| {
                    // ui.set_height(30.);

                    // let screen_rect = ui.max_rect();
                    // ui.set_clip_rect(screen_rect);
                    ui.spacing_mut().item_spacing.y = 0.;
                    egui::ScrollArea::vertical()
                        .id_source("first")
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .stick_to_bottom(true)
                        //.max_height(10.)
                        .vertical_scroll_offset(5.)
                        .min_scrolled_height(20.)
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            UISpriteText::from_sections([
                                UISpriteTextSection::new("Charmander", &font_assets.text)
                                    .with_color(Color32::BLUE),
                                UISpriteTextSection::new(" used ", &font_assets.text),
                                UISpriteTextSection::new("AncientPower", &font_assets.text)
                                    .with_color(Color32::GREEN),
                                UISpriteTextSection::new("!", &font_assets.text),
                            ])
                            .show(ui);

                            UISpriteText::from_sections([UISpriteTextSection::new(
                                "It's supper effective!",
                                &font_assets.text,
                            )])
                            .show(ui);

                            UISpriteText::from_sections([
                                UISpriteTextSection::new("Rattata", &font_assets.text)
                                    .with_color(Color32::LIGHT_BLUE),
                                UISpriteTextSection::new(" took ", &font_assets.text),
                                UISpriteTextSection::new("26", &font_assets.text)
                                    .with_color(Color32::LIGHT_BLUE),
                                UISpriteTextSection::new(" damage!", &font_assets.text),
                            ])
                            .show(ui);
                        });
                });
        });

    // egui::CentralPanel::default()
    //     // Because it covers the whole screen, make sure that it doesn't overlay the egui background frame
    //     .frame(egui::Frame::none())
    //     .show(ctx, |ui| {
    //         // Get the screen rect
    //         let screen_rect = ui.max_rect();
    //         // Calculate a margin of 15% of the screen size
    //         let outer_margin = screen_rect.size() * 0.15;
    //         let outer_margin = UiRect {
    //             left: Val::Px(outer_margin.x),
    //             right: Val::Px(outer_margin.x),
    //             // Make top and bottom margins smaller
    //             top: Val::Px(outer_margin.y / 2.0),
    //             bottom: Val::Px(outer_margin.y / 2.0),
    //         };

    //         // ui.label("world");
    //         // egui::SidePanel::left("SidePanel").default_width(300.).show(ctx, |ui| {
    //         //     ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
    //         //     ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
    //         //         UISpriteText::from_sections([
    //         //             UISpriteTextSection{
    //         //                 value: "Lorem ipsum".to_string(),
    //         //                 color: Color32::RED,
    //         //                 font: &font_assets.text
    //         //             },
    //         //             UISpriteTextSection{
    //         //                 value: " dolor sit amet,".to_string(),
    //         //                 color: Color32::WHITE,
    //         //                 font: &font_assets.text
    //         //             },
    //         //             UISpriteTextSection{
    //         //                 value: " consectetur adipiscing elit.".to_string(),
    //         //                 color: Color32::BLUE,
    //         //                 font: &font_assets.text
    //         //             }
    //         //         ]).show(ui);
    //         //         ui.sprite_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.", &font_assets.text);
    //         //         // ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.");
    //         //         // ui.label("World!");
    //         //     });
    //         // });

    //         // 0.33333334 0.6666666 0.125 0.375
    //         // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
    //         //     ui.set_min_height(80.);
    //         //     ui.set_max_height(80.);
    //         // });
    //     });
}

fn spawn_test(font_assets: Res<FontAssets>, mut commands: Commands) {
    let text_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        ..default()
    };
    let text_red_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        color: Color::srgb(1.0, 0.0, 0.0), // RED
        ..default()
    };
    let text_blue_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        color: Color::srgb(0.44, 0.59, 0.89), // Approximation of 7098e3 in hex
        ..default()
    };

    commands.spawn((
        WorldNumber {
            value: 10,
            r#type: WorldNumberType::Damage,
        },
        SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 20.))),
    ));

    // commands
    //     .spawn(Text2DSpriteBundle {
    //         transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
    //         text_anchor: bevy::sprite::Anchor::BottomLeft,
    //         text: SpriteText {
    //             sections: [
    //                 SpriteTextSection::new("Lorem ipsum dolor sit amet, ", text_red_style.clone()),
    //                 SpriteTextSection::new("consectetur ", text_blue_style.clone()),
    //                 SpriteTextSection::new("adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.", text_style.clone())
    //             ].to_vec(),
    //             ..default()
    //         },
    //         // text: SpriteText::from_section("Lorem ipsum dolor sit amet, ", text_style.clone()),
    //         text_2d_bounds: Text2dBounds {size: Vec2::new(200., 300.)},
    //         ..default()
    //     })
    //     .insert(Name::new("TextSprite Test"));
}
