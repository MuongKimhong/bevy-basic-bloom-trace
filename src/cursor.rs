use crate::circle;
use crate::Lifetime;
use bevy::prelude::*;

pub fn screen_coord_to_world_coord(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_moved: &CursorMoved,
) -> Option<(f32, f32)> {
    if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_moved.position)
    {
        return Some((world_position.origin.x, world_position.origin.y));
    }
    None
}

pub fn on_cursor_move_event(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut evr_cursor: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    for ev in evr_cursor.read() {
        if let Some((camera, camera_transform)) = q_camera.iter().next() {
            let (x, y) = screen_coord_to_world_coord(&camera, camera_transform, ev).unwrap();

            commands.spawn((
                circle::create_circle_mesh(&mut meshes, &mut materials, x, y),
                Lifetime {
                    timer: Timer::from_seconds(0.4, TimerMode::Once),
                },
            ));
        }
    }
}
