pub mod circle;
pub mod cursor;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

#[derive(Component)]
pub struct Lifetime {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, cursor::on_cursor_move_event)
        .add_systems(Update, remove_circle_mesh_after_timeout)
        .run();
}

fn create_camera_2d_bundle() -> Camera2dBundle {
    Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    }
}

fn remove_circle_mesh_after_timeout(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        // Update the timer
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((create_camera_2d_bundle(), BloomSettings::default()));
}
