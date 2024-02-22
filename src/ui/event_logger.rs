use std::collections::VecDeque;
use std::fmt::format;

use bevy::prelude::*;
use bevy_egui::egui::Color32;
use bevy_egui::{egui, EguiContexts};
use egui::TextBuffer;

use crate::actions::walk_action::WalkAction;
use crate::actions::ActionExecutedEvent;
use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::assets::ui_assets::UIAssets;
use crate::graphics::ui::{BorderedFrame, SpriteTextEguiUiExt, UISpriteText, UISpriteTextSection};

#[derive(Default, Resource)]
pub struct EventLogs {
    pub logs: VecDeque<String>,
}

pub(crate) fn gather_logs(
    mut ev_action_executed: EventReader<ActionExecutedEvent>,
    mut event_logs: ResMut<EventLogs>,
) {
    for action_executed in ev_action_executed.read() {
        info!("Gather logs -> {:?}", action_executed.action);
        let action = action_executed.action.as_any();

        if let Some(walk_action) = action.downcast_ref::<WalkAction>() {
            event_logs.logs.push_front(format!(
                "{:?} took a walk to {:?}",
                walk_action.entity, walk_action.to
            ));
            continue;
        };
    }
}

pub(crate) fn event_logger_ui(
    mut ctx: EguiContexts,
    event_logs: Res<EventLogs>,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UIAssets>,
) {
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
                    ui.spacing_mut().item_spacing.y = 0.;
                    egui::ScrollArea::vertical()
                        .id_source("first")
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .stick_to_bottom(true)
                        .min_scrolled_height(20.)
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            for event_log in event_logs.logs.iter() {
                                ui.sprite_text(event_log, &font_assets.text);
                            }

                            // UISpriteText::from_sections([
                            //     UISpriteTextSection::new("Charmander", &font_assets.text)
                            //         .with_color(Color32::BLUE),
                            //     UISpriteTextSection::new(" used ", &font_assets.text),
                            //     UISpriteTextSection::new("AncientPower", &font_assets.text)
                            //         .with_color(Color32::GREEN),
                            //     UISpriteTextSection::new("!", &font_assets.text),
                            // ])
                            // .show(ui);

                            // UISpriteText::from_sections([UISpriteTextSection::new(
                            //     "It's supper effective!",
                            //     &font_assets.text,
                            // )])
                            // .show(ui);

                            // UISpriteText::from_sections([
                            //     UISpriteTextSection::new("Rattata", &font_assets.text)
                            //         .with_color(Color32::LIGHT_BLUE),
                            //     UISpriteTextSection::new(" took ", &font_assets.text),
                            //     UISpriteTextSection::new("26", &font_assets.text)
                            //         .with_color(Color32::LIGHT_BLUE),
                            //     UISpriteTextSection::new(" damage!", &font_assets.text),
                            // ])
                            // .show(ui);
                        });
                });
        });
}
