use bevy_inspector_egui::prelude::*;
use egui::Color32;
use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::actions::damage_action::DamageAction;
use crate::actions::death_action::DeathAction;
use crate::actions::spell_action::SpellAction;
use crate::actions::walk_action::WalkAction;
use crate::actions::ActionExecutedEvent;
use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::assets::ui_assets::UIAssets;
use crate::graphics::ui::{BorderedFrame, UISpriteText, UISpriteTextSection};

const SCROLL_SPEED: f32 = 15.;

#[derive(Default, Resource)]
pub struct EventLogs {
    pub logs: VecDeque<EventLogLine>,
}

#[derive(Default, InspectorOptions)]
pub enum EventLogColor {
    TeamLeader, // #009CFF
    TeamMember, // #FFFF00
    Friendly,   // #FFFF00
    Foe,        // #00ffff
    Spell,
    Damage,
    #[default]
    None,
}

impl EventLogColor {
    pub fn to_color32(&self) -> Color32 {
        match self {
            EventLogColor::TeamLeader => Color32::from_rgb(0, 157, 255),
            EventLogColor::TeamMember => Color32::from_rgb(255, 255, 0),
            EventLogColor::Friendly => Color32::from_rgb(255, 255, 0),
            EventLogColor::Foe => Color32::from_rgb(0, 255, 255),
            EventLogColor::Spell => Color32::from_rgb(255, 0, 0),
            EventLogColor::Damage => Color32::from_rgb(0, 255, 255),
            EventLogColor::None => Color32::WHITE,
        }
    }
}

#[derive(Default, InspectorOptions)]
pub struct EventLogLineSection {
    text: String,
    color: EventLogColor,
}
impl EventLogLineSection {
    pub fn new(text: String, color: EventLogColor) -> Self {
        Self { text, color }
    }
}

#[derive(Default, InspectorOptions)]
pub struct EventLogLine(Vec<EventLogLineSection>);

pub(crate) fn gather_logs(
    mut ev_action_executed: EventReader<ActionExecutedEvent>,
    name_query: Query<&Name>,
    mut event_logs: ResMut<EventLogs>,
) {
    for action_executed in ev_action_executed.read() {
        info!("Gather logs -> {:?}", action_executed.action);
        let action = action_executed.action.as_any();
        let entity_name = name_query.get(action_executed.entity).unwrap().as_str();

        if let Some(walk_action) = action.downcast_ref::<WalkAction>() {
            let log_line_sections = vec![
                EventLogLineSection::new(entity_name.to_string(), EventLogColor::TeamLeader),
                EventLogLineSection::new(
                    format!(" walk to {:?}!", walk_action.to),
                    EventLogColor::None,
                ),
            ];
            event_logs.logs.push_back(EventLogLine(log_line_sections));
            continue;
        };
        if let Some(spell_action) = action.downcast_ref::<SpellAction>() {
            let log_line_sections = vec![
                EventLogLineSection::new(entity_name.to_string(), EventLogColor::TeamLeader),
                EventLogLineSection::new(" used ".to_string(), EventLogColor::None),
                EventLogLineSection::new(spell_action.spell.name.to_string(), EventLogColor::Spell),
            ];
            event_logs.logs.push_back(EventLogLine(log_line_sections));
            continue;
        }
        if let Some(damage_action) = action.downcast_ref::<DamageAction>() {
            let entity_name = name_query.get(damage_action.target).unwrap().as_str();

            let log_line_sections = vec![
                EventLogLineSection::new(entity_name.to_string(), EventLogColor::Foe),
                EventLogLineSection::new(" took ".to_string(), EventLogColor::None),
                EventLogLineSection::new(damage_action.value.to_string(), EventLogColor::Damage),
                EventLogLineSection::new(" damage!".to_string(), EventLogColor::None),
            ];
            event_logs.logs.push_back(EventLogLine(log_line_sections));
            continue;
        }

        if let Some(death_action) = action.downcast_ref::<DeathAction>() {
            let entity_name = name_query.get(death_action.target).unwrap().as_str();

            let log_line_sections = vec![
                EventLogLineSection::new(entity_name.to_string(), EventLogColor::Foe),
                EventLogLineSection::new(" was defeated!".to_string(), EventLogColor::None),
            ];
            event_logs.logs.push_back(EventLogLine(log_line_sections));
            continue;
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ScrollAnimation {
    current_scroll_position: f32,
    target_offset: f32,
}

const TEXT_LINE_HEIGHT: f32 = 12.;

pub(crate) fn event_logger_ui(
    mut ctx: EguiContexts,
    event_logs: Res<EventLogs>,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UIAssets>,
    time: Res<Time>,
    mut scroll_animation: Local<ScrollAnimation>,
) {
    let ctx = ctx.ctx_mut();

    if event_logs.is_changed() && event_logs.logs.len() > 3 {
        scroll_animation.target_offset += TEXT_LINE_HEIGHT;
    }

    if scroll_animation.target_offset > 0. {
        let delta = time.delta_seconds() * SCROLL_SPEED;
        let prev_current_offset = scroll_animation.target_offset;
        scroll_animation.target_offset -= delta;

        let diff = (prev_current_offset - scroll_animation.target_offset).max(0.);

        scroll_animation.current_scroll_position += diff;
    }

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
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .stick_to_bottom(true)
                        .min_scrolled_height(20.)
                        .vertical_scroll_offset(scroll_animation.current_scroll_position + 2.)
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            for event_line_log in event_logs.logs.iter() {
                                let sprite_text_sections = event_line_log
                                    .0
                                    .iter()
                                    .map(|section| {
                                        UISpriteTextSection::new(&section.text, &font_assets.text)
                                            .with_color(section.color.to_color32())
                                    })
                                    .collect::<Vec<_>>();
                                UISpriteText::from_sections(sprite_text_sections).show(ui);
                            }
                        });
                });
        });
}
