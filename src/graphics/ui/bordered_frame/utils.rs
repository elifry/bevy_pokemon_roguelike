use bevy_egui::egui::{self, Pos2, Rect, TextureId, Vec2};

pub fn build_nine_patch_mesh(
    dest_rect: Rect,
    texture: TextureId,
    atlas_size: Pos2,
    texture_size: Rect,
    texture_border_size: egui::style::Margin,
) -> egui::Mesh {
    let white = egui::Color32::WHITE;

    // Texture coordinates for the sub-image within the atlas
    let tx0 = texture_size.min.x / atlas_size.x;
    let ty0 = texture_size.min.y / atlas_size.y;
    let tx1 = (texture_size.min.x + texture_size.width()) / atlas_size.x;
    let ty1 = (texture_size.min.y + texture_size.height()) / atlas_size.y;

    // UV coordinates for the 9-patch borders, relative to the sub-image
    let buv = egui::style::Margin {
        left: texture_border_size.left / texture_size.width(),
        right: texture_border_size.right / texture_size.width(),
        top: texture_border_size.top / texture_size.height(),
        bottom: texture_border_size.bottom / texture_size.height(),
    };

    // Convert UV border margins to texture UV coordinates
    let uv_left = tx0 + buv.left * (tx1 - tx0);
    let uv_right = tx1 - buv.right * (tx1 - tx0);
    let uv_top = ty0 + buv.top * (ty1 - ty0);
    let uv_bottom = ty1 - buv.bottom * (ty1 - ty0);

    let b = texture_border_size;

    // Build the 9-patches
    let mut mesh = egui::Mesh {
        texture_id: texture,
        ..Default::default()
    };

    // Top left
    mesh.add_rect_with_uv(
        Rect::from_min_size(dest_rect.min, Vec2::new(b.left, b.top)),
        egui::Rect::from_min_max(Pos2::new(tx0, ty0), Pos2::new(uv_left, uv_top)),
        white,
    );
    // Top center
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            dest_rect.min + Vec2::new(b.left, 0.0),
            Vec2::new(dest_rect.width() - b.left - b.right, b.top),
        ),
        egui::Rect::from_min_max(Pos2::new(uv_left, ty0), Pos2::new(uv_left, uv_top)),
        white,
    );
    // Top right
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            Pos2::new(dest_rect.max.x - b.right, dest_rect.min.y),
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
            dest_rect.min + Vec2::new(0.0, b.top),
            Vec2::new(b.left, dest_rect.height() - b.top - b.bottom),
        ),
        egui::Rect::from_min_max(Pos2::new(tx0, uv_top), Pos2::new(uv_left, uv_bottom)),
        white,
    );
    // Middle center
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            dest_rect.min + Vec2::new(b.left, b.top),
            Vec2::new(
                dest_rect.width() - b.left - b.right,
                dest_rect.height() - b.top - b.bottom,
            ),
        ),
        egui::Rect::from_min_max(Pos2::new(uv_left, uv_top), Pos2::new(uv_right, uv_bottom)),
        white,
    );
    // Middle right
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            dest_rect.min + Vec2::new(dest_rect.width() - b.right, b.top),
            Vec2::new(b.right, dest_rect.height() - b.top - b.bottom),
        ),
        egui::Rect::from_min_max(Pos2::new(uv_right, uv_top), Pos2::new(tx1, uv_bottom)),
        white,
    );
    // Bottom left
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            dest_rect.min + Vec2::new(0.0, dest_rect.height() - b.bottom),
            Vec2::new(b.left, b.bottom),
        ),
        egui::Rect::from_min_max(Pos2::new(tx0, uv_bottom), Pos2::new(uv_left, ty1)),
        white,
    );
    // Bottom center
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            dest_rect.min + Vec2::new(b.left, dest_rect.height() - b.bottom),
            Vec2::new(dest_rect.width() - b.left - b.right, b.bottom),
        ),
        egui::Rect::from_min_max(Pos2::new(uv_left, uv_bottom), Pos2::new(uv_right, ty1)),
        white,
    );
    // Bottom right
    mesh.add_rect_with_uv(
        Rect::from_min_size(
            dest_rect.min + Vec2::new(dest_rect.width() - b.right, dest_rect.height() - b.bottom),
            Vec2::new(b.right, b.bottom),
        ),
        egui::Rect::from_min_max(Pos2::new(uv_right, uv_bottom), Pos2::new(tx1, ty1)),
        white,
    );

    mesh
}
