use std::path::Path;

use bevy::prelude::*;

pub fn get_path_from_handle(handle: &UntypedHandle) -> Option<&Path> {
    handle.path().and_then(|p| Some(p.path()))
}
