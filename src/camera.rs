use crate::{graphics::TILE_SIZE, player::Player, GameState};
use bevy::{prelude::*, render::camera::ScalingMode};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_2d_camera)
            .add_systems(Update, camera_follow.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct UserInterfaceCamera;

#[derive(Component)]
pub struct Orthographic2DCamera;

fn spawn_2d_camera(mut commands: Commands) {
    let orthographic_projection = OrthographicProjection {
        scale: 0.5,
        near: -30.,
        scaling_mode: ScalingMode::FixedVertical(640.),
        ..default()
    };
    let mut camera = Camera2dBundle {
        projection: orthographic_projection,
        ..default()
    };
    // camera.transform.translation = Vec3::new(
    //     4. * TILE_SIZE,
    //     4. * TILE_SIZE,
    //     camera.transform.translation.z,
    // );
    // camera.projection.scale = 0.5;
    // camera.projection.scaling_mode = ScalingMode::FixedVertical(320.);
    commands.spawn(camera).insert(Orthographic2DCamera);
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Orthographic2DCamera>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
