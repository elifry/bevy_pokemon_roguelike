use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_egui::egui::Color32;
use bevy_egui::{egui, EguiContexts};

use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::assets::ui_assets::UIAssets;
use crate::graphics::ui::sprite_text::{
    SpriteText, SpriteTextSection, SpriteTextStyle, Text2DSpriteBundle,
};
use crate::graphics::ui::{BorderedFrame, SpriteTextEguiUiExt, UISpriteText, UISpriteTextSection};
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test)
            .add_systems(Update, ui.run_if(in_state(GameState::Playing)));
    }
}

fn ui(mut ctx: EguiContexts, font_assets: Res<FontAssets>, ui_assets: Res<UIAssets>) {
    let ctx = ctx.ctx_mut();
    egui::CentralPanel::default()
        // Because it covers the whole screen, make sure that it doesn't overlay the egui background frame
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            // Get the screen rect
            let screen_rect = ui.max_rect();
            // Calculate a margin of 15% of the screen size
            let outer_margin = screen_rect.size() * 0.15;
            let outer_margin = UiRect {
                left: Val::Px(outer_margin.x),
                right: Val::Px(outer_margin.x),
                // Make top and bottom margins smaller
                top: Val::Px(outer_margin.y / 2.0),
                bottom: Val::Px(outer_margin.y / 2.0),
            };

            // ui.label("world");
            egui::SidePanel::left("SidePanel").default_width(300.).show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    UISpriteText::from_sections([
                        UISpriteTextSection{
                            value: "Lorem ipsum".to_string(),
                            color: Color32::RED,
                            font: &font_assets.text
                        },
                        UISpriteTextSection{
                            value: " dolor sit amet,".to_string(),
                            color: Color32::WHITE,
                            font: &font_assets.text
                        },
                        UISpriteTextSection{
                            value: " consectetur adipiscing elit.".to_string(),
                            color: Color32::BLUE,
                            font: &font_assets.text
                        }
                    ]).show(ui);
                    ui.sprite_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.", &font_assets.text);
                    // ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.");
                    // ui.label("World!");
                });
            });


            // egui::Grid::new("some_unique_id").show(ui, |ui| {
            //     ui.sprite_text("Hello ", &font_assets.text);
            //     ui.sprite_colored_label("World!", Color32::BLUE, &font_assets.text);
            //     ui.end_row();
            // });

            ui.vertical_centered(|ui| {
                BorderedFrame::new(&ui_assets.menu).padding(UiRect::all(Val::Px(7.))).show(ui, |ui| {
                    ui.sprite_text("Hello world!", &font_assets.text);
                });
            });

            // ui.vertical_centered(|ui| {
            //     ui.retro_label("Hello World UI", &font_assets.text);
            // });
        });
}

fn spawn_test(font_assets: Res<FontAssets>, mut commands: Commands) {
    let text_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        ..default()
    };
    let text_red_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        color: Color::RED,
        ..default()
    };
    let text_blue_style = SpriteTextStyle {
        font: font_assets.text.clone(),
        color: Color::hex("7098e3").unwrap(),
        ..default()
    };

    commands
        .spawn(Text2DSpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
            text_anchor: bevy::sprite::Anchor::BottomLeft,
            text: SpriteText {
                sections: [
                    SpriteTextSection::new("Lorem ipsum dolor sit amet, ", text_red_style.clone()),
                    SpriteTextSection::new("consectetur ", text_blue_style.clone()),
                    SpriteTextSection::new("adipiscing elit. Aenean ullamcorper scelerisque odio nec rutrum. Sed facilisis blandit mauris a vehicula. Praesent sagittis diam eget pulvinar elementum.", text_style.clone())
                ].to_vec(),
                ..default()
            },
            // text: SpriteText::from_section("Lorem ipsum dolor sit amet, ", text_style.clone()),
            text_2d_bounds: Text2dBounds {size: Vec2::new(200., 300.)},
            ..default()
        })
        .insert(Name::new("TextSprite Test"));
}
