use std::path::Path;

use bevy::prelude::*;

pub fn get_path_from_handle(handle: &UntypedHandle) -> Option<&Path> {
    handle.path().map(|p| p.path())
}

pub fn find_first_handle_by_extension<'a, A: Asset>(
    handles: &'a [UntypedHandle],
    extension: &'a str,
) -> Option<Handle<A>>
where
    A: 'static,
{
    handles.iter().find_map(|handle| {
        let path = get_path_from_handle(handle)?;
        if path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
            Some(handle.clone().typed::<A>())
        } else {
            None
        }
    })
}
