use crate::graphics::TILE_SIZE;
use bevy::{prelude::*, render::camera::ScalingMode};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle { ..default() };
    camera.transform.translation = Vec3::new(
        4. * TILE_SIZE,
        4. * TILE_SIZE,
        camera.transform.translation.z,
    );
    // camera.projection.scale = 0.5;
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 320.,
        min_height: 320.,
    };
    commands.spawn(camera);
}
