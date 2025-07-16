//! 9-Patch style bordered frame rendering
// Adapted from <https://docs.rs/egui/0.18.1/src/egui/containers/frame.rs.html>

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::{self, epaint};

use crate::graphics::ui::bordered_frame::utils::build_nine_patch_mesh;

use super::{BorderImage, BorderImageBackground};

/// A 9-patch style bordered frame.
///
/// # See Also
///
/// - [`UiBorderImage`]
pub struct BorderedFrame {
    texture: egui::TextureId,
    texture_size: egui::Rect,
    texture_border_size: epaint::Margin,
    atlas_size: egui::Pos2,
    padding: epaint::Margin,
    margin: epaint::Margin,
    background: Option<BorderedFrameBackground>,
}

#[derive(Debug, Clone)]
pub struct BorderedFrameBackground {
    texture: egui::TextureId,
    texture_size: egui::Rect,
    atlas_size: egui::Pos2,
}

impl BorderedFrame {
    /// Create a new frame with the given [`BorderImage`]
    #[must_use = "You must call .show() to render the frame"]
    pub fn new(style: &BorderImage) -> Self {
        let s = style.texture_size;
        let b = style.texture_border_size;
        Self {
            texture: style.egui_texture,
            texture_size: egui::Rect::from_min_max(
                egui::Pos2::new(s.min.x as f32, s.min.y as f32),
                egui::Pos2::new(s.max.x as f32, s.max.y as f32),
            ),
            texture_border_size: epaint::Margin {
                left: if let Val::Px(px) = b.left { px } else { 0. },
                right: if let Val::Px(px) = b.right { px } else { 0. },
                top: if let Val::Px(px) = b.top { px } else { 0. },
                bottom: if let Val::Px(px) = b.bottom { px } else { 0. },
            },
            atlas_size: egui::Pos2::new(style.atlas_size.x as f32, style.atlas_size.y as f32),
            padding: Default::default(),
            margin: Default::default(),
            background: None,
        }
    }

    /// Set the padding. This will be applied on the inside of the border.
    #[must_use = "You must call .show() to render the frame"]
    pub fn padding(mut self, margin: UiRect) -> Self {
        self.padding = epaint::Margin {
            left: if let Val::Px(px) = margin.left {
                px
            } else {
                0.
            },
            right: if let Val::Px(px) = margin.right {
                px
            } else {
                0.
            },
            top: if let Val::Px(px) = margin.top { px } else { 0. },
            bottom: if let Val::Px(px) = margin.bottom {
                px
            } else {
                0.
            },
        };

        self
    }

    /// Set the margin. This will be applied on the outside of the border.
    #[must_use = "You must call .show() to render the frame"]
    pub fn background(mut self, background: &BorderImageBackground) -> Self {
        self.background = Some(BorderedFrameBackground {
            texture: background.egui_texture,
            texture_size: egui::Rect::from_min_max(
                egui::Pos2::new(
                    background.texture_size.min.x as f32,
                    background.texture_size.min.y as f32,
                ),
                egui::Pos2::new(
                    background.texture_size.max.x as f32,
                    background.texture_size.max.y as f32,
                ),
            ),
            atlas_size: egui::Pos2::new(
                background.atlas_size.x as f32,
                background.atlas_size.y as f32,
            ),
        });

        self
    }

    /// Set the margin. This will be applied on the outside of the border.
    #[must_use = "You must call .show() to render the frame"]
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = epaint::Margin {
            left: if let Val::Px(px) = margin.left {
                px
            } else {
                0.
            },
            right: if let Val::Px(px) = margin.right {
                px
            } else {
                0.
            },
            top: if let Val::Px(px) = margin.top { px } else { 0. },
            bottom: if let Val::Px(px) = margin.bottom {
                px
            } else {
                0.
            },
        };

        self
    }

    /// Render the frame
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<R> {
        self.show_dyn(ui, Box::new(add_contents))
    }

    fn show_dyn<'c, R>(
        self,
        ui: &mut egui::Ui,
        add_contents: Box<dyn FnOnce(&mut egui::Ui) -> R + 'c>,
    ) -> egui::InnerResponse<R> {
        let mut prepared = self.begin(ui);
        let ret = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ui);

        egui::InnerResponse {
            inner: ret,
            response,
        }
    }

    fn begin(self, ui: &mut egui::Ui) -> BorderedFramePrepared {
        let background_shape_idx = ui.painter().add(egui::Shape::Noop);

        let mut content_rect = ui.available_rect_before_wrap();
        content_rect.min += self.padding.left_top() + self.margin.left_top();
        content_rect.max -= self.padding.right_bottom() + self.margin.right_bottom();

        // Avoid negative size
        content_rect.max.x = content_rect.max.x.max(content_rect.min.x);
        content_rect.max.y = content_rect.max.y.max(content_rect.min.y);

        let content_ui = ui.child_ui(content_rect, *ui.layout());

        BorderedFramePrepared {
            frame: self,
            background_shape_idx,
            content_ui,
        }
    }

    pub fn paint(&self, paint_rect: egui::Rect) -> egui::Shape {
        let border_mesh = build_nine_patch_mesh(
            paint_rect,
            self.texture,
            self.atlas_size,
            self.texture_size,
            self.texture_border_size,
        );

        let mut shapes = vec![egui::Shape::Mesh(border_mesh)];

        if let Some(ref background) = self.background {
            let background_mesh = build_nine_patch_mesh(
                paint_rect,
                background.texture,
                background.atlas_size,
                background.texture_size,
                self.texture_border_size,
            );

            shapes.push(egui::Shape::Mesh(background_mesh));
        }
        egui::Shape::Vec(shapes)
    }
}

/// Internal helper struct for rendering the [`BorderedFrame`]
struct BorderedFramePrepared {
    frame: BorderedFrame,
    background_shape_idx: egui::layers::ShapeIdx,
    content_ui: egui::Ui,
}

impl BorderedFramePrepared {
    fn end(self, ui: &mut egui::Ui) -> egui::Response {
        use egui::Vec2;

        let min_rect = self.content_ui.min_rect();
        let m = self.frame.padding;
        let paint_rect = egui::Rect {
            min: min_rect.min - Vec2::new(m.left, m.top),
            max: min_rect.max + Vec2::new(m.right, m.bottom),
        };
        if ui.is_rect_visible(paint_rect) {
            let shape = self.frame.paint(paint_rect);
            ui.painter().set(self.background_shape_idx, shape);
        }

        ui.allocate_rect(paint_rect, egui::Sense::hover())
    }
}
