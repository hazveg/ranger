use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorCoordinates(pub Vec3);

pub fn update_cursor_position(
    mut res_cursor_coordinates: ResMut<CursorCoordinates>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Err(_) = camera_query.get_single() {
        return;
    }
    
    if let Err(_) = window_query.get_single() {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_coordinates) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        res_cursor_coordinates.0 = world_coordinates.extend(0.0);
        println!("{}", res_cursor_coordinates.0);
    }
}
