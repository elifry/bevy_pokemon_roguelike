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
    commands.spawn((
        Camera2d,
        Msaa::Off,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 640.,
            },
            scale: 0.5,
            near: -30.,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
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
