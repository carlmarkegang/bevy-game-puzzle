use crate::{generate_random_int, setupcamera};
use bevy::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub vel_x: f32,
    pub vel_y: f32,
    pub size: f32,
}

pub fn setup_brick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Brick
    let brick_size: f32 = 20.;
    for _i in 0..10 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.5))),
            Transform::from_translation(Vec3::new(
                0. as f32,
                generate_random_int(0..500) as f32,
                10.0,
            ))
            .with_scale(Vec2::splat(brick_size).extend(1.)),
            Brick {
                vel_x: 0.0,
                vel_y: 0.0,
                size: brick_size,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }
}

pub fn brick_controls(mut query: Query<&mut Brick>) {
    let jump_power = 3.0;
    let speed = 1.0;
    for mut brick in query.iter_mut() {
        let generate_rand = generate_random_int(1..200);
        if generate_rand == 1 {
            brick.vel_x = -speed;
        }

        if generate_rand == 2 {
            brick.vel_x = speed;
        }

        if generate_rand == 3 {
            brick.vel_x = 0.0;
        }

    }
}

pub fn brick_movements(mut brick_query: Query<(&mut Transform, &mut Brick)>) {
    for (mut transform, mut brick) in brick_query.iter_mut() {
        transform.translation.x += brick.vel_x;
        if transform.translation.y >= -90.0 {
            if brick.vel_y > -2.0 {
                brick.vel_y -= 0.1;
            }
        } else {
            transform.translation.y = -90.0;
        }
        transform.translation.y += brick.vel_y;
    }
}

pub fn collision_check_brick(mut query_brick: Query<(&mut Transform, &mut Brick)>) {
    for (obstacle_transform, obstacle) in query_brick.iter_mut() {
    for (mut brick_transform, mut brick) in query_brick.iter_mut() {
        
            if std::ptr::eq(&*brick, &*obstacle) {
                continue;
            }

            let brick_position = brick_transform.translation;
            let obstacle_position = obstacle_transform.translation;

            let distance = brick_position.distance(obstacle_position);
            let brick_radius = brick.size / 2.;
            let obstacle_radius = obstacle.size / 2.;
            if distance < brick_radius + obstacle_radius {
                let shift_vector = brick_position - obstacle_position;
                let shift_distance = brick_radius + obstacle_radius - distance;
                let shift = shift_vector.normalize() * shift_distance;

                brick_transform.translation.x += shift.x;
                brick_transform.translation.y += shift.y;
                brick.vel_y = 0.0;
            }
        }
    }
}
