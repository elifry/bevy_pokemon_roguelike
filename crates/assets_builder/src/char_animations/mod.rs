mod anim_data;
mod char_animation;
pub mod orientation;

use std::{
    fs::{self, File},
    path::Path,
};

use bevy_math::{IVec2, URect, UVec2};
use image::{ImageBuffer, Rgba};

use crate::char_animations::char_animation::IVec2Serialized;

use self::{
    anim_data::{AnimData, AnimInfo},
    char_animation::CharAnimation,
};

pub fn create_char_animation(source_directory: &Path, output_filename: &str) {
    println!("Creating {output_filename}");
    let output_path = Path::new(output_filename);
    fs::create_dir_all(output_path.parent().unwrap().to_str().unwrap()).unwrap();

    let anim_data_path = source_directory.join("AnimData.xml");
    let anim_data_content = fs::read(anim_data_path).expect("Failed to read AnimData.xml");
    let anim_data =
        AnimData::parse_from_xml(&anim_data_content).expect("Failed to parse AnimData.xml");

    // let font_texture_files = list_png_files_in_folder(source_directory)
    //     .unwrap_or_else(|_| panic!("Unable to list texture files in {:?}", source_directory));

    for (anim_key, _) in anim_data.anims.anim.iter() {
        let anim_info = anim_data.get(anim_key);
        let anim_key_str: &'static str = anim_info.value().name.into();

        let offsets_texture_file = source_directory.join(format!("{anim_key_str}-Offsets.png"));
        let offsets_texture = image::open(offsets_texture_file).unwrap().to_rgba8();

        let columns = anim_info.columns();
        let rows = anim_info.rows();

        let mut body_offsets = vec![vec![IVec2Serialized::default(); columns]; rows];
        let mut head_offsets = vec![vec![IVec2Serialized::default(); columns]; rows];
        let mut right_offsets = vec![vec![IVec2Serialized::default(); columns]; rows];
        let mut left_offsets = vec![vec![IVec2Serialized::default(); columns]; rows];

        for row in 0..(rows - 1) {
            for column in 0..(columns - 1) {
                let tile_size = anim_info.tile_size();
                let texture_rect = URect::from_corners(
                    UVec2::new(tile_size.x * column as u32, tile_size.y * row as u32),
                    UVec2::new(
                        tile_size.x * column as u32 + tile_size.x,
                        tile_size.y * row as u32 + tile_size.y,
                    ),
                );

                let offsets = extract_offsets(&anim_info, &offsets_texture, texture_rect);
                body_offsets[row][column] = offsets.body;
                head_offsets[row][column] = offsets.head;
                right_offsets[row][column] = offsets.right;
                left_offsets[row][column] = offsets.left;
            }
        }

        let animation_texture_file = source_directory.join(format!("{anim_key_str}-Anim.png"));
        let animation_texture = image::open(animation_texture_file).unwrap().to_rgba8();

        let durations = anim_info
            .value()
            .durations
            .duration
            .iter()
            .map(|d| d.value)
            .collect::<Vec<_>>();

        let char_animation = CharAnimation {
            // texture: animation_texture.to_vec(),
            texture: vec![],
            index: anim_info.index(),
            frame_width: anim_info.tile_size().x,
            frame_height: anim_info.tile_size().y,
            durations,
            rush_frame: anim_info.value().rush_frame,
            hit_frame: anim_info.value().hit_frame,
            return_frame: anim_info.value().return_frame,
            shadow_offsets: vec![],
            body_offsets,
            head_offsets,
            left_offsets,
            right_offsets,
        };

        let mut char_animation_file =
            File::create(format!("{output_filename}-{anim_key}.xml")).unwrap();

        let _ = char_animation.save(&mut char_animation_file);
    }
}

#[derive(Default, Debug)]
struct Offsets {
    body: IVec2Serialized,  // Green
    head: IVec2Serialized,  // Black
    right: IVec2Serialized, // Blue
    left: IVec2Serialized,  // Red
}

fn extract_offsets(
    anim_info: &AnimInfo,
    atlas_image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: URect,
) -> Offsets {
    let tile_size = anim_info.tile_size();

    let mut offsets = Offsets::default();

    for y in (texture.min.y)..texture.max.y {
        for x in (texture.min.x)..texture.max.x {
            // Access individual color components
            let pixel = atlas_image.get_pixel(x, y);

            let real_x: i32 = (x - texture.min.x).try_into().unwrap();
            let real_y: i32 = (y - texture.min.y).try_into().unwrap();

            if *pixel == Rgba([0, 0, 0, 255]) {
                offsets.head = calculate_offset(real_x, real_y, tile_size).into();
            } else if *pixel == Rgba([255, 0, 0, 255]) {
                offsets.left = calculate_offset(real_x, real_y, tile_size).into();
            } else if *pixel == Rgba([0, 255, 0, 255]) {
                offsets.body = calculate_offset(real_x, real_y, tile_size).into();
            } else if *pixel == Rgba([0, 0, 255, 255]) {
                offsets.right = calculate_offset(real_x, real_y, tile_size).into();
            }
        }
    }

    offsets
}

fn calculate_offset(real_x: i32, real_y: i32, tile_size: UVec2) -> IVec2 {
    let half_tile_size = tile_size / 2;
    let coordinates = IVec2::new(real_x, tile_size.y as i32 - real_y);
    coordinates - IVec2::new(half_tile_size.x as i32, half_tile_size.y as i32)
}
