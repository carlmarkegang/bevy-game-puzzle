use crate::{generate_random_int, setupcamera};
use bevy::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub id: i32,
    pub brick_type: i32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub size: f32,
    pub time_still: f32,
    pub time_still_move_x: f32,
    pub time_still_move_y: f32,
    pub to_delete: i32,
}

#[derive(Component)]
pub struct BrickCompare {
    pub id: i32,
    pub size: f32,
    pub brick_type: i32,
}

const MAX_TIME_STILL: f32 = 100.;
const MAX_BRICKS: i32 = 20;

pub fn setup_brick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Brick
    let brick_size: f32 = 20.;
    for _i in 0..MAX_BRICKS {
        let random_brick_type = generate_random_int(1..4);
        let mut color_r = generate_random_int(0..100) as f32 / 100.0;
        let mut color_g = generate_random_int(0..100) as f32 / 100.0;
        let mut color_b = generate_random_int(0..100) as f32 / 100.0;

        if random_brick_type == 1 {
            color_r = 1.0;
            color_g = 0.2;
            color_b = 0.2;
        }

        if random_brick_type == 2 {
            color_r = 0.2;
            color_g = 1.0;
            color_b = 0.2;
        }

        if random_brick_type == 3 {
            color_r = 0.2;
            color_g = 0.2;
            color_b = 1.0;
        }

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
            Transform::from_translation(Vec3::new(
                generate_random_int(-50..50) as f32,
                _i as f32 * 200.0,
                10.0,
            ))
            .with_scale(Vec2::splat(brick_size).extend(1.)),
            Brick {
                id: _i,
                brick_type: random_brick_type,
                vel_x: 0.0,
                vel_y: 0.0,
                size: brick_size,
                time_still: 0.0,
                time_still_move_x: 0.0,
                time_still_move_y: 0.0,
                to_delete: 0,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.5))),
            Transform::from_translation(Vec3::new(0., 0., 20.0))
                .with_scale(Vec2::splat(0.0).extend(1.)),
            BrickCompare {
                id: _i,
                brick_type: random_brick_type,
                size: brick_size,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }
}

pub fn brick_movements(mut brick_query: Query<(&mut Transform, &mut Brick)>) {
    let max_speed = -2.0;
    for (mut transform, mut brick) in brick_query.iter_mut() {
        if brick.time_still >= MAX_TIME_STILL {
            brick.vel_y = 0.0;
            continue;
        }

        transform.translation.x += brick.vel_x;

        if brick.vel_y > max_speed {
            brick.vel_y -= 0.1;
        }

        if transform.translation.y <= -50.0 {
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

pub fn collision_check_brick(
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    mut query_brick_compare: Query<(&mut Transform, &mut BrickCompare), Without<Brick>>,
) {
    for (mut brick_transform, mut brick) in query_brick.iter_mut() {
        for (mut obstacle_transform, obstacle) in query_brick_compare.iter_mut() {
            if obstacle.id == brick.id {
                continue;
            }

            //if brick.time_still >= 500. {
            //    brick_transform.translation.x = generate_random_int(-100..100) as f32;
            //    brick_transform.translation.y = 200.0;
            //    brick.time_still = 0.0;
            //}

            if brick.time_still >= MAX_TIME_STILL {
                //println!("time still: {}", brick.time_still);
                brick.vel_y = 0.0;
                brick.vel_x = 0.0;
                brick_transform.translation.x = brick.time_still_move_x;
                brick_transform.translation.y = brick.time_still_move_y;
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
                brick.vel_y = -1.0;
            }
        }
    }
}

pub fn set_pos_compare_brick(
    mut query_brick_compare: Query<(&mut Transform, &mut BrickCompare)>,
    query_brick: Query<(&mut Transform, &mut Brick), Without<BrickCompare>>,
) {
    for (brick_transform, brick) in query_brick.iter() {
        for (mut brick_transform_compare, brick_compare) in query_brick_compare.iter_mut() {
            if brick_compare.id == brick.id {
                brick_transform_compare.translation = brick_transform.translation;
            }
        }
    }
}

pub fn time_still_check(mut query_brick: Query<(&mut Transform, &mut Brick)>) {
    for (mut brick_transform, mut brick) in query_brick.iter_mut() {
        //let rounded_x = (brick_transform.translation.x * 10.0).round() / 10.0;
        //let rounded_y = (brick_transform.translation.y * 10.0).round() / 10.0;

        let rounded_x = (brick_transform.translation.x).round();
        let rounded_y = (brick_transform.translation.y).round();

        if rounded_x == brick.time_still_move_x && rounded_y == brick.time_still_move_y {
            brick.time_still += 1.0;
        } else {
            //println!("time still: {} id: {}", brick.time_still, brick.id);
            brick.time_still = 0.0;
        }
        brick.time_still_move_x = rounded_x;
        brick.time_still_move_y = rounded_y;
    }
}

/// graph traversal ??
pub fn check_touching(
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    mut query_brick_compare: Query<(&mut Transform, &mut BrickCompare), Without<Brick>>,
) {
    let mut current_id = 0;
    let mut touching_array_depth_1: Vec<i32> = Vec::new();
    let mut touching_array_depth_2: Vec<i32> = Vec::new();
    let mut touching_array_depth_3: Vec<i32> = Vec::new();
    let mut touching_array_depth_4: Vec<i32> = Vec::new();

    for _i in 0..MAX_BRICKS {
        current_id = _i;
        touching_array_depth_1 = Vec::new();
        touching_array_depth_2 = Vec::new();
        touching_array_depth_3 = Vec::new();

        let mut reset_run = false;
        for (mut brick_transform, mut brick) in query_brick.iter_mut() {
            if brick.time_still > 6000. {
                reset_run = true;
            }

            if brick.time_still < 600. && reset_run == false {
                return;
            }
        }

        // If nothing happened for 2 minutes
        if reset_run == true {
            for (mut brick_transform, mut brick) in query_brick.iter_mut() {
                brick.to_delete = 1;
            }
        }

        // Depth 1
        for (mut brick_transform, mut brick) in query_brick.iter_mut() {
            for (mut obstacle_transform, obstacle) in query_brick_compare.iter_mut() {
                if obstacle.id == brick.id {
                    continue;
                }

                let brick_position = brick_transform.translation;
                let obstacle_position = obstacle_transform.translation;

                let distance = brick_position.distance(obstacle_position);
                let brick_radius = brick.size / 1.9;
                let obstacle_radius = obstacle.size / 1.9;

                if distance < brick_radius + obstacle_radius {
                    if brick.brick_type == obstacle.brick_type {
                        if !touching_array_depth_1.contains(&brick.id) {
                            touching_array_depth_1.push(brick.id);
                        }
                        if !touching_array_depth_1.contains(&obstacle.id)
                            || !touching_array_depth_2.contains(&obstacle.id)
                            || !touching_array_depth_3.contains(&obstacle.id)
                        {
                            touching_array_depth_1.push(obstacle.id);
                        }
                    }
                }
            }
        }

        // Depth 2
        for (mut brick_transform, mut brick) in query_brick.iter_mut() {
            for (mut obstacle_transform, obstacle) in query_brick_compare.iter_mut() {
                if obstacle.id == brick.id {
                    continue;
                }

                if !touching_array_depth_1.contains(&brick.id) {
                    continue;
                }

                let brick_position = brick_transform.translation;
                let obstacle_position = obstacle_transform.translation;

                let distance = brick_position.distance(obstacle_position);
                let brick_radius = brick.size / 1.9;
                let obstacle_radius = obstacle.size / 1.9;

                if distance < brick_radius + obstacle_radius {
                    if brick.brick_type == obstacle.brick_type {
                        if !touching_array_depth_1.contains(&obstacle.id)
                            || !touching_array_depth_2.contains(&obstacle.id)
                            || !touching_array_depth_3.contains(&obstacle.id)
                        {
                            touching_array_depth_2.push(obstacle.id);
                        }
                    }
                }
            }
        }

        // Depth 3
        for (mut brick_transform, mut brick) in query_brick.iter_mut() {
            for (mut obstacle_transform, obstacle) in query_brick_compare.iter_mut() {
                if obstacle.id == brick.id {
                    continue;
                }

                if !touching_array_depth_1.contains(&brick.id) {
                    continue;
                }

                if !touching_array_depth_2.contains(&brick.id) {
                    continue;
                }

                let brick_position = brick_transform.translation;
                let obstacle_position = obstacle_transform.translation;

                let distance = brick_position.distance(obstacle_position);
                let brick_radius = brick.size / 1.9;
                let obstacle_radius = obstacle.size / 1.9;

                if distance < brick_radius + obstacle_radius {
                    if brick.brick_type == obstacle.brick_type {
                        if !touching_array_depth_1.contains(&obstacle.id)
                        || !touching_array_depth_2.contains(&obstacle.id)
                        || !touching_array_depth_3.contains(&obstacle.id)
                    {
                            touching_array_depth_3.push(obstacle.id);
                        }
                    }
                }
            }
        }

        if touching_array_depth_3.len() >= 1 {
            for (mut brick_transform, mut brick) in query_brick.iter_mut() {
                if touching_array_depth_1.contains(&brick.id)
                    || touching_array_depth_2.contains(&brick.id)
                    || touching_array_depth_3.contains(&brick.id)
                {
                    brick.to_delete = 1;
                }
            }
        }
    }
}

pub fn delete_touching(
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    mut query_brick_compare: Query<(&mut Transform, &mut BrickCompare), Without<Brick>>,
) {
    let mut was_deleted = false;
    for (mut brick_transform, mut brick) in query_brick.iter_mut() {
        if brick.to_delete == 1 {
            brick_transform.translation.x = generate_random_int(-100..100) as f32;
            brick_transform.translation.y = generate_random_int( 500..2000) as f32;
            brick.time_still_move_x = brick_transform.translation.x;
            brick.time_still_move_y = brick_transform.translation.y;
            brick.time_still = 0.0;
            brick.brick_type = generate_random_int(1..3);
            brick.to_delete = 0;
            brick.vel_y = 0.0;
            brick.vel_x = 0.0;
            was_deleted = true;
        }
    }

    for (mut brick_transform, mut brick) in query_brick.iter_mut() {
        if was_deleted == true {
            brick.time_still = 0.;
        }
    }
}
