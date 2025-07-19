use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::GameState;

use super::{
    assets::font_assets::FontAssets,
    ui::{SpriteText, SpriteTextStyle, Text2DSpriteBundle},
};

pub struct WorldNumberPlugin;

impl Plugin for WorldNumberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_world_number_render, animate_world_number)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .register_type::<WorldNumber>()
        .register_type::<AnimatedWorldNumber>();
    }
}

#[derive(Debug, Reflect, Default)]
pub enum WorldNumberType {
    #[default]
    Damage,
    Heal,
    Exp,
}

#[derive(Component, Debug, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
pub struct WorldNumber {
    pub value: i32,
    pub r#type: WorldNumberType,
}

#[derive(Component, Debug, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
struct AnimatedWorldNumber {
    timer: Timer,
}

fn spawn_world_number_render(
    font_assets: Res<FontAssets>,
    mut commands: Commands,
    query: Query<(Entity, &WorldNumber, &Transform), Added<WorldNumber>>,
) {
    for (entity, world_number, transform) in query.iter() {
        let font = match world_number.r#type {
            WorldNumberType::Damage => &font_assets.damage,
            WorldNumberType::Heal => &font_assets.heal,
            WorldNumberType::Exp => &font_assets.exp,
        };
        let sign = match world_number.value.is_positive() {
            true => "+",
            false => "",
        };
        let text_style = SpriteTextStyle {
            font: font.clone(),
            ..default()
        };

        commands.entity(entity).insert((
            Text2DSpriteBundle {
                transform: transform.with_translation(Vec3::new(0., 15., 10.)),
                text_anchor: bevy::sprite::Anchor::Center,
                text: SpriteText::from_section(format!("{sign}{}", world_number.value), text_style),
                visibility: Visibility::Visible,
                ..default()
            },
            AnimatedWorldNumber {
                timer: Timer::from_seconds(1., TimerMode::Once),
            },
        ));
    }
}

fn animate_world_number(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut AnimatedWorldNumber,
        &mut Transform,
        &mut Sprite,
    )>,
    mut commands: Commands,
) {
    for (entity, mut animated, mut transform, mut sprite) in query.iter_mut() {
        animated.timer.tick(time.delta());

        if animated.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
        // transform.translation.x = (time.elapsed_seconds() * 40.).cos() * 0.6;
        transform.translation.y += time.delta_secs() * 10.;

        if animated.timer.fraction() > 0.5 {
            let [red, green, blue, alpha] = sprite.color.to_srgba().to_f32_array();
            let alpha = (alpha - time.delta_secs() * 1.).max(0.);
            sprite.color = Color::srgba(red, green, blue, alpha);
        }
    }
}
