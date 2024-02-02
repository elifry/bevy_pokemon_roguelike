use std::path::Path;

use bevy::prelude::*;

pub fn get_path_from_handle(handle: &UntypedHandle) -> Option<&Path> {
    handle.path().map(|p| p.path())
}
