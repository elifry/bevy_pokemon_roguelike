use bevy::{asset::LoadState, prelude::*};

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_systems(OnEnter(GameState::Initializing), set_playing)
            .add_systems(
                Update,
                check_assets_loading.run_if(in_state(GameState::Loading)),
            );
    }
}

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

fn set_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn check_assets_loading(
    mut next_state: ResMut<NextState<GameState>>,
    loading: Res<AssetsLoading>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let mut is_loading: bool = false;

    for asset in loading.0.iter() {
        match asset_server.get_load_state(asset.id()) {
            Some(LoadState::Loading) => {
                is_loading = true;
                break;
            }
            Some(LoadState::Failed) => {
                error!("asset loading error");
            }
            _ => {}
        }
    }

    if is_loading {
        return;
    }

    info!("Assets loaded");
    commands.remove_resource::<AssetsLoading>();
    next_state.set(GameState::AssetsLoaded);
}
