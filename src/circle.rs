use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

pub const CIRCLE_RADIUS: f32 = 10.0;

pub const PURPLE_BLOOM: Color = Color::srgb(5.0, 0.0, 5.0);
pub const BLUE_BLOOM: Color = Color::srgb(5.0, 0.0, 3.5);
pub const PINK_BLOOM: Color = Color::srgb(5.0, 1.0, 5.0);
pub const CYAN_BLOOM: Color = Color::srgb(0.0, 5.0, 5.0);

pub const BLOOM_COLORS: [Color; 4] = [PURPLE_BLOOM, BLUE_BLOOM, PINK_BLOOM, CYAN_BLOOM];

pub fn get_random_color() -> Color {
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..BLOOM_COLORS.len());
    BLOOM_COLORS[random_index]
}

pub fn create_circle_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(CIRCLE_RADIUS)).into(),
        material: materials.add(get_random_color()),
        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
        ..default()
    }
}
