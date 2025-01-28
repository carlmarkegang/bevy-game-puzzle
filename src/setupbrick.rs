use crate::{generate_random_int, setupcamera};
use bevy::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub id: i32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub size: f32,
}

#[derive(Component)]
pub struct BrickCompare {
    pub id: i32,
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
    let brick_size: f32 = 30.;
    for _i in 0..50 {
            let random_colorR = generate_random_int(0..100) as f32 / 100.0 ;
            let random_colorB = generate_random_int(0..100) as f32 / 100.0 ;
            let random_colorG = generate_random_int(0..100) as f32 / 100.0 ;
        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(random_colorR, random_colorG, random_colorB))),
            Transform::from_translation(Vec3::new(
                generate_random_int(-50..50) as f32,
                _i  as f32 * 100.0,
                10.0,
            ))
            .with_scale(Vec2::splat(brick_size).extend(1.)),
            Brick {
                id: _i,
                vel_x: 0.0,
                vel_y: 0.0,
                size: brick_size,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));


        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.5))),
            Transform::from_translation(Vec3::new(
                0.,
                0.,
                20.0,
            ))
            .with_scale(Vec2::splat(0.0).extend(1.)),
            BrickCompare {
                id: _i,
                vel_x: 0.0,
                vel_y: 0.0,
                size: brick_size,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }
}

pub fn brick_controls(mut query: Query<&mut Brick>) {
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
    let max_speed = -2.0;
    for (mut transform, mut brick) in brick_query.iter_mut() {
        transform.translation.x += brick.vel_x;
        if transform.translation.y >= -50.0 {
            if brick.vel_y > max_speed {
                brick.vel_y -= 0.1;
            }
        } else {
            transform.translation.y = -50.0;
        }
        transform.translation.y += brick.vel_y;

        if transform.translation.x < -50.0 {
            transform.translation.x = -50.0;
        } 
        
        if transform.translation.x > 50.0 {
            transform.translation.x = 50.0;
        } 
    }
}

pub fn pos_check_brick(
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    query_brick_compare: Query<(&mut Transform, &mut BrickCompare), Without<Brick>>
) {
    return;

    for (mut brick_transform, brick) in query_brick.iter_mut() {
        for (brick_transform_compare, brick_compare) in query_brick_compare.iter() {

            //println!("Brick ID: {}, Obstacle ID: {}", brick.id, brick_compare.id);

            if brick_compare.id == brick.id {
                continue;
            }

            if brick_transform.translation.x == brick_transform_compare.translation.x {
                brick_transform.translation.x += 0.01;
            }
        }
    }
}


pub fn collision_check_brick(    
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    query_brick_compare: Query<(&mut Transform, &mut BrickCompare), Without<Brick>>
) {
    for (mut brick_transform, mut brick) in query_brick.iter_mut() {
        for (obstacle_transform, obstacle) in query_brick_compare.iter() {

            if obstacle.id == brick.id {
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
                if brick_transform.translation.y > -50.0 {
                    brick_transform.translation.y += shift.y;
                    brick.vel_y = -1.0;
                }

            }
        }
    }
}
 


 pub fn set_pos_compare_brick(
    mut query_brick_compare: Query<(&mut Transform, &mut BrickCompare)>,
    query_brick: Query<(&mut Transform, &mut Brick), Without<BrickCompare>>
) {
    for (brick_transform, brick) in query_brick.iter() {
        for (mut brick_transform_compare, brick_compare) in query_brick_compare.iter_mut() {
            if brick_compare.id == brick.id {
                brick_transform_compare.translation = brick_transform.translation;
            }

        }
    }
}