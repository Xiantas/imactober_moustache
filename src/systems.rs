use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        projection: bevy::prelude::OrthographicProjection {
            near: -1000.0,
            scaling_mode: bevy::render::camera::ScalingMode::Fixed{
                width: 1.,
                height: 1.,
            },
            scale: 3.0,
            ..default()
        },
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.5)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
    });
}
