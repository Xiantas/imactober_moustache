use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;

use crate::resources::MouseInfos;
use crate::components::FurPiece;

use crate::pseudo_random::random_u128;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., -1., 0.),
        projection: bevy::prelude::OrthographicProjection {
            near: -1000.0,
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 16.,
                height: 9.,
            },
            ..default()
        },
        ..default()
    });

    for i in -30..30i32 {
        for j in -15..15i32 {
            commands.spawn((
                FurPiece {
                    sheared: false,
                    velocity: Vec3::new(
                        ((random_u128()%10) as i32 -5) as f32 * 0.005,
                        ((random_u128()%10) as i32 + 2) as f32 * 0.005,
                        0.0
                    ),
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::from_xyz(i as f32 * 0.2, j as f32 * 0.2, 0.).with_scale(Vec3::splat(0.3)),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    ..default()
                }
            ));
        }
    }
}

pub fn fur_activation(
    mut furs: Query<(&mut FurPiece, &mut Transform)>,
    mut mouse: ResMut<MouseInfos>,
) {
    if let Some(m_pos) = mouse.pos {
        if mouse.clicking {
            for (mut fur, mut tr) in &mut furs {
                if tr.translation.x - tr.scale.x/2. < m_pos.x &&
                    m_pos.x < tr.translation.x + tr.scale.x/2. &&
                    tr.translation.y - tr.scale.y/2. < m_pos.y &&
                    m_pos.y < tr.translation.y + tr.scale.y/2.
                {
                    fur.sheared = true;
                }
            }
        }
    }

    mouse.clicking = false;
}

pub fn fur_gravity_update(
    mut furs: Query<(&mut FurPiece, &mut Transform)>
) {
    for (mut fur, mut transform) in &mut furs {
        if fur.sheared {
            transform.translation += fur.velocity;
            fur.velocity += Vec3::new(0.0, -0.01, 0.0);
        }
    }
}

//TODO remove unseen furs

pub fn clicks_handeling(
    mut mouse: ResMut<MouseInfos>,
    clicks: Res<Input<MouseButton>>,
) {
    mouse.clicking |= clicks.pressed(MouseButton::Left)
}

pub fn mouse_pos_updater(
    mut mouse: ResMut<MouseInfos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    mouse.pos = window.cursor_position()
        .and_then(|cur_pos| camera.viewport_to_world_2d(camera_transform, cur_pos))
        .map(|v2| v2.extend(0.0));
}
