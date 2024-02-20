//! 9-Patch style bordered frame rendering
// Adapted from <https://docs.rs/egui/0.18.1/src/egui/containers/frame.rs.html>

use bevy::prelude::*;
use bevy_egui::egui::{self, Pos2};

use super::BorderImage;

/// A 9-patch style bordered frame.
///
/// # See Also
///
/// - [`UiBorderImage`]
pub struct BorderedFrame {
    bg_texture: egui::TextureId,
    texture_size: egui::Rect,
    texture_border_size: egui::style::Margin,
    atlas_size: egui::Pos2,
    padding: egui::style::Margin,
    margin: egui::style::Margin,
    border_only: bool,
}

impl BorderedFrame {
    /// Create a new frame with the given [`BorderImage`]
    #[must_use = "You must call .show() to render the frame"]
    pub fn new(style: &BorderImage) -> Self {
        let s = style.texture_size;
        let b = style.texture_border_size;
        Self {
            bg_texture: style.egui_texture,
            texture_size: egui::Rect::from_min_max(
                egui::Pos2::new(s.min.x as f32, s.min.y as f32),
                egui::Pos2::new(s.max.x as f32, s.max.y as f32),
            ),
            texture_border_size: egui::style::Margin {
                left: if let Val::Px(px) = b.left { px } else { 0. },
                right: if let Val::Px(px) = b.right { px } else { 0. },
                top: if let Val::Px(px) = b.top { px } else { 0. },
                bottom: if let Val::Px(px) = b.bottom { px } else { 0. },
            },
            atlas_size: egui::Pos2::new(style.atlas_size.x as f32, style.atlas_size.y as f32),
            padding: Default::default(),
            margin: Default::default(),
            border_only: false,
        }
    }

    /// Set the padding. This will be applied on the inside of the border.
    #[must_use = "You must call .show() to render the frame"]
    pub fn padding(mut self, margin: UiRect) -> Self {
        self.padding = egui::style::Margin {
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
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = egui::style::Margin {
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

    /// If border_only is set to `true`, then the middle section of the frame will be transparent,
    /// only the border will be rendered.
    #[must_use = "You must call .show() to render the frame"]
    pub fn border_only(mut self, border_only: bool) -> Self {
        self.border_only = border_only;

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
        use egui::{Pos2, Rect, Vec2};
        let white = egui::Color32::WHITE;

        let mut mesh = egui::Mesh {
            texture_id: self.bg_texture,
            ..Default::default()
        };

        // Texture coordinates for the sub-image within the atlas
        let tx0 = self.texture_size.min.x / self.atlas_size.x;
        let ty0 = self.texture_size.min.y / self.atlas_size.y;
        let tx1 = (self.texture_size.min.x + self.texture_size.width()) / self.atlas_size.x;
        let ty1 = (self.texture_size.min.y + self.texture_size.height()) / self.atlas_size.y;

        // UV coordinates for the 9-patch borders, relative to the sub-image
        let buv = egui::style::Margin {
            left: self.texture_border_size.left / self.texture_size.width(),
            right: self.texture_border_size.right / self.texture_size.width(),
            top: self.texture_border_size.top / self.texture_size.height(),
            bottom: self.texture_border_size.bottom / self.texture_size.height(),
        };

        // Convert UV border margins to texture UV coordinates
        let uv_left = tx0 + buv.left * (tx1 - tx0);
        let uv_right = tx1 - buv.right * (tx1 - tx0);
        let uv_top = ty0 + buv.top * (ty1 - ty0);
        let uv_bottom = ty1 - buv.bottom * (ty1 - ty0);

        let b = self.texture_border_size;

        // Build the 9-patches

        // Top left
        mesh.add_rect_with_uv(
            Rect::from_min_size(paint_rect.min, Vec2::new(b.left, b.top)),
            egui::Rect::from_min_size(Pos2::new(tx0, ty0), Vec2::new(uv_left, uv_top)),
            white,
        );
        // Top center
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                paint_rect.min + Vec2::new(b.left, 0.0),
                Vec2::new(paint_rect.width() - b.left - b.right, b.top),
            ),
            egui::Rect::from_min_size(Pos2::new(uv_left, ty0), Vec2::new(uv_left, uv_top)),
            white,
        );
        // Top right
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                Pos2::new(paint_rect.max.x - b.right, paint_rect.min.y),
                Vec2::new(b.right, b.top),
            ),
            Rect::from_min_max(
                Pos2::new(uv_right, ty0), // Start right before the right border, top aligned
                Pos2::new(tx1, uv_top), // End at the extreme right of the texture portion, bottom aligned to top border's bottom
            ),
            white,
        );
        // Middle left
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                paint_rect.min + Vec2::new(0.0, b.top),
                Vec2::new(b.left, paint_rect.height() - b.top - b.bottom),
            ),
            egui::Rect::from_min_max(Pos2::new(tx0, uv_top), Pos2::new(uv_left, uv_bottom)),
            white,
        );
        // Middle center
        if !self.border_only {
            mesh.add_rect_with_uv(
                Rect::from_min_size(
                    paint_rect.min + Vec2::new(b.left, b.top),
                    Vec2::new(
                        paint_rect.width() - b.left - b.right,
                        paint_rect.height() - b.top - b.bottom,
                    ),
                ),
                egui::Rect::from_min_max(
                    Pos2::new(uv_left, uv_top),
                    Pos2::new(uv_right, uv_bottom),
                ),
                white,
            );
        }
        // Middle right
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                paint_rect.min + Vec2::new(paint_rect.width() - b.right, b.top),
                Vec2::new(b.right, paint_rect.height() - b.top - b.bottom),
            ),
            egui::Rect::from_min_max(Pos2::new(uv_right, uv_top), Pos2::new(tx1, uv_bottom)),
            white,
        );
        // // Bottom left
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                paint_rect.min + Vec2::new(0.0, paint_rect.height() - b.bottom),
                Vec2::new(b.left, b.bottom),
            ),
            egui::Rect::from_min_max(Pos2::new(tx0, uv_bottom), Pos2::new(uv_left, ty1)),
            white,
        );
        // Bottom center
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                paint_rect.min + Vec2::new(b.left, paint_rect.height() - b.bottom),
                Vec2::new(paint_rect.width() - b.left - b.right, b.bottom),
            ),
            egui::Rect::from_min_max(Pos2::new(uv_left, uv_bottom), Pos2::new(uv_right, ty1)),
            white,
        );
        // Bottom right
        mesh.add_rect_with_uv(
            Rect::from_min_size(
                paint_rect.min
                    + Vec2::new(paint_rect.width() - b.right, paint_rect.height() - b.bottom),
                Vec2::new(b.right, b.bottom),
            ),
            egui::Rect::from_min_max(Pos2::new(uv_right, uv_bottom), Pos2::new(tx1, ty1)),
            white,
        );

        egui::Shape::Mesh(mesh)
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
