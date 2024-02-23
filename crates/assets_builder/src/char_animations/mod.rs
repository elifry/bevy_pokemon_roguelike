mod anim_data;

use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use ::char_animation::{
    orientation::Orientation, CharAnimation, CharAnimationEntry, IVec2Serialized,
};
use bevy_math::{IVec2, URect, UVec2};
use image::{ImageBuffer, Rgba};

use self::anim_data::{AnimData, AnimInfo};

pub fn create_char_animation(source_directory: &Path, output_filename: &str) {
    println!("Creating {output_filename}");
    let output_path = Path::new(output_filename);
    fs::create_dir_all(output_path.parent().unwrap().to_str().unwrap()).unwrap();

    let anim_data_path = source_directory.join("AnimData.xml");
    let anim_data_content = fs::read(anim_data_path).expect("Failed to read AnimData.xml");
    let anim_data =
        AnimData::parse_from_xml(&anim_data_content).expect("Failed to parse AnimData.xml");

    let mut char_animation_entries = HashMap::new();

    for (anim_key, _) in anim_data.anims.anim.iter() {
        let anim_info = anim_data.get(anim_key);
        let anim_key_str: &'static str = anim_info.value().name.into();

        let offsets_texture_file = source_directory.join(format!("{anim_key_str}-Offsets.png"));
        let offsets_texture = image::open(offsets_texture_file).unwrap().to_rgba8();

        let shadow_texture_file = source_directory.join(format!("{anim_key_str}-Shadow.png"));
        let shadow_texture = image::open(shadow_texture_file).unwrap().to_rgba8();

        let mut body_offsets = HashMap::new();
        let mut head_offsets = HashMap::new();
        let mut right_offsets = HashMap::new();
        let mut left_offsets = HashMap::new();
        let mut shadow_offsets = HashMap::new();

        let columns = anim_info.columns();
        let orientations: Box<dyn Iterator<Item = Orientation>> = anim_info.orientations();

        for (row, orientation) in orientations.enumerate() {
            body_offsets.insert(
                orientation.clone(),
                vec![IVec2Serialized::default(); columns],
            );
            head_offsets.insert(
                orientation.clone(),
                vec![IVec2Serialized::default(); columns],
            );
            right_offsets.insert(
                orientation.clone(),
                vec![IVec2Serialized::default(); columns],
            );
            left_offsets.insert(
                orientation.clone(),
                vec![IVec2Serialized::default(); columns],
            );
            shadow_offsets.insert(
                orientation.clone(),
                vec![IVec2Serialized::default(); columns],
            );

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
                body_offsets.get_mut(&orientation).unwrap()[column] = offsets.body;
                head_offsets.get_mut(&orientation).unwrap()[column] = offsets.head;
                right_offsets.get_mut(&orientation).unwrap()[column] = offsets.right;
                left_offsets.get_mut(&orientation).unwrap()[column] = offsets.left;

                shadow_offsets.get_mut(&orientation).unwrap()[column] =
                    extract_shadow_offset(&anim_info, &shadow_texture, texture_rect);
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

        let char_animation_entry = CharAnimationEntry {
            texture: animation_texture.to_vec(),
            index: anim_info.index(),
            frame_width: anim_info.tile_size().x,
            frame_height: anim_info.tile_size().y,
            durations,
            rush_frame: anim_info.value().rush_frame,
            hit_frame: anim_info.value().hit_frame,
            return_frame: anim_info.value().return_frame,
            shadow_offsets,
            body_offsets,
            head_offsets,
            left_offsets,
            right_offsets,
        };

        char_animation_entries.insert(*anim_key, char_animation_entry);
    }

    let mut char_animation_file = File::create(output_filename).unwrap();

    let char_animation = CharAnimation {
        anim: char_animation_entries,
    };
    let _ = char_animation.save(&mut char_animation_file);
}

#[derive(Default, Debug)]
struct Offsets {
    body: IVec2Serialized,  // Green
    head: IVec2Serialized,  // Black
    right: IVec2Serialized, // Blue
    left: IVec2Serialized,  // Red
}

fn extract_shadow_offset(
    anim_info: &AnimInfo,
    atlas_image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: URect,
) -> IVec2Serialized {
    let tile_size = anim_info.tile_size();

    for y in (texture.min.y)..texture.max.y {
        for x in (texture.min.x)..texture.max.x {
            // Access individual color components
            let pixel = atlas_image.get_pixel(x, y);

            let real_x: i32 = (x - texture.min.x).try_into().unwrap();
            let real_y: i32 = (y - texture.min.y).try_into().unwrap();

            if *pixel == Rgba([255, 255, 255, 255]) {
                return calculate_offset(real_x, real_y, tile_size).into();
            }
        }
    }

    panic!("Unable to find the shadow offsets for {:?}", anim_info);
}

fn extract_offsets(
    anim_info: &AnimInfo,
    atlas_image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    texture: URect,
) -> Offsets {
    let tile_size = anim_info.tile_size();

    let mut offsets = Offsets::default();

    let mut part_counter = 0;
    for y in (texture.min.y)..texture.max.y {
        for x in (texture.min.x)..texture.max.x {
            // Access individual color components
            let pixel = atlas_image.get_pixel(x, y);

            let real_x: i32 = (x - texture.min.x).try_into().unwrap();
            let real_y: i32 = (y - texture.min.y).try_into().unwrap();

            if *pixel == Rgba([0, 0, 0, 255]) {
                offsets.head = calculate_offset(real_x, real_y, tile_size).into();
                part_counter += 1;
            } else if *pixel == Rgba([255, 0, 0, 255]) {
                offsets.left = calculate_offset(real_x, real_y, tile_size).into();
                part_counter += 1;
            } else if *pixel == Rgba([0, 255, 0, 255]) {
                offsets.body = calculate_offset(real_x, real_y, tile_size).into();
                part_counter += 1;
            } else if *pixel == Rgba([0, 0, 255, 255]) {
                offsets.right = calculate_offset(real_x, real_y, tile_size).into();
                part_counter += 1;
            }
        }
    }

    if part_counter != 4 {
        panic!(
            "Could'nt find all the offsets part {part_counter} for {:?}",
            anim_info
        );
    }

    offsets
}

fn calculate_offset(real_x: i32, real_y: i32, tile_size: UVec2) -> IVec2 {
    let half_tile_size = tile_size / 2;
    let coordinates = IVec2::new(real_x, tile_size.y as i32 - real_y);
    coordinates - IVec2::new(half_tile_size.x as i32, half_tile_size.y as i32)
}
