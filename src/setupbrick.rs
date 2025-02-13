use std::collections::{HashMap, HashSet};

use crate::{generate_random_int, setupcamera, MousePos, PointsText};
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
    pub game_lost: bool,
    pub set_game_lost_text: bool,
}

#[derive(Component)]
pub struct BrickCompare {
    pub id: i32,
    pub size: f32,
    pub brick_type: i32,
}

const MAX_TIME_STILL: f32 = 100.;
const MAX_BRICKS: i32 = 20;
pub const BRICK_SIZE: f32 = 20.;

pub fn setup_brick(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Brick
    for _i in 0..MAX_BRICKS {}
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

        if transform.translation.y <= -80.0 {
            transform.translation.y = -80.0;
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
        for (obstacle_transform, obstacle) in query_brick_compare.iter_mut() {
            if obstacle.id == brick.id {
                continue;
            }

            if brick.time_still >= MAX_TIME_STILL {
                brick.vel_y = 0.0;
                brick.vel_x = 0.0;
                brick_transform.translation.x = brick.time_still_move_x;
                brick_transform.translation.y = brick.time_still_move_y;
                if brick.time_still_move_y > 80.0 {
                    brick.game_lost = true;
                }
                continue;
            }

            let brick_position = brick_transform.translation;
            let obstacle_position = obstacle_transform.translation;

            let distance = brick_position.distance(obstacle_position);
            let brick_radius = brick.size / 2.;
            let obstacle_radius = obstacle.size / 2.;
            if distance < brick_radius + obstacle_radius {
                if distance > 0.0 {
                    // Avoid NaN issues
                    let shift_vector = (brick_position - obstacle_position).normalize();
                    //let shift_distance = (brick_radius + obstacle_radius - distance) / 2.0; // Dampening effect
                    let shift_distance = brick_radius + obstacle_radius - distance; // Full push instead of half
                    brick_transform.translation.x += shift_vector.x * shift_distance;
                    brick_transform.translation.y += shift_vector.y * shift_distance;
                    //obstacle_transform.translation -= shift_vector * shift_distance;
                    //brick.vel_y = -1.0;
                }
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
    for (brick_transform, mut brick) in query_brick.iter_mut() {
        let rounded_x = (brick_transform.translation.x * 10.0).round() / 10.0;
        let rounded_y = (brick_transform.translation.y * 10.0).round() / 10.0;
        //let rounded_x = (brick_transform.translation.x).round();
        //let rounded_y = (brick_transform.translation.y).round();

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

pub fn check_touching(
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    query_brick_compare: Query<(&mut Transform, &mut BrickCompare), Without<Brick>>,
) {
    let mut to_delete_list: Vec<i32> = Vec::new();
    let mut connections: HashMap<i32, Vec<i32>> = HashMap::new();

    for (brick_transform, brick) in query_brick.iter_mut() {
        for (obstacle_transform, obstacle) in query_brick_compare.iter() {
            let brick_position = brick_transform.translation;
            let obstacle_position = obstacle_transform.translation;

            let distance = brick_position.distance(obstacle_position);
            let brick_radius = brick.size / 1.9;
            let obstacle_radius = obstacle.size / 1.9;

            if distance < brick_radius + obstacle_radius && brick.brick_type == obstacle.brick_type
            {
                connections.entry(brick.id).or_default().push(obstacle.id);
                connections.entry(obstacle.id).or_default().push(brick.id); // Bidirectional connection
            }
        }
    }

    // DFS function to explore the graph
    fn dfs(
        id: i32,
        connections: &HashMap<i32, Vec<i32>>,
        visited: &mut HashSet<i32>,
        cluster_size: &mut usize,
    ) {
        if visited.contains(&id) {
            return;
        }
        visited.insert(id);
        *cluster_size += 1;

        if let Some(neighbors) = connections.get(&id) {
            for &neighbor in neighbors {
                dfs(neighbor, connections, visited, cluster_size);
            }
        }
    }

    // Traverse each brick
    for (brick_transform, brick) in query_brick.iter_mut() {
        let mut visited = HashSet::new();
        let mut cluster_size = 0;

        dfs(brick.id, &connections, &mut visited, &mut cluster_size);

        // If the cluster is big enough, mark all bricks in it for deletion
        if cluster_size >= 4 {
            to_delete_list.extend(visited);
        }
    }

    for (brick_transform, mut brick) in query_brick.iter_mut() {
        if to_delete_list.contains(&brick.id) {
            brick.to_delete = 1;
        }
    }
}

pub fn delete_touching(
    mut query_brick: Query<(Entity, &mut Transform, &mut Brick)>,
    mut query_brick_compare: Query<(Entity, &mut Transform, &mut BrickCompare), Without<Brick>>,
    mut commands: Commands,
    mut textquery: Query<(&mut Text, &mut PointsText)>,
) {
    let mut was_deleted = false;
    for (brick_entity, brick_transform, brick) in query_brick.iter_mut() {
        if brick.to_delete == 1 {
            for (brick_entity_compare, brick_transform_compare, brick_compare) in
                query_brick_compare.iter_mut()
            {
                if brick_compare.id == brick.id {
                    commands.entity(brick_entity_compare).despawn();
                }
            }

            commands.entity(brick_entity).despawn();
            was_deleted = true;
        }
    }

    for (brick_entity, brick_transform, mut brick) in query_brick.iter_mut() {
        if was_deleted == true {
            // Unlock everything after remove
            // brick.time_still = 0.;

            if brick.game_lost == false {
                for (span, mut points_text) in textquery.iter_mut() {
                    points_text.points += 500;
                }
            }
        }

        // Display win message
        if brick.game_lost == true && brick.set_game_lost_text == false {
            commands.spawn((
                Text::new("Sorry you lost!"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px((setupcamera::RES_WIDTH / 2) as f32),
                    left: Val::Px((setupcamera::RES_WIDTH / 2) as f32),
                    ..default()
                },
                setupcamera::PIXEL_PERFECT_LAYERS,
            ));
            brick.set_game_lost_text = true;
        }
    }
}

pub fn spawn_brick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut query_brick: Query<(&mut Transform, &mut Brick)>,
    mut query: Query<&mut MousePos>,
    mut textquery: Query<&mut PointsText>,
) {
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    let mut clicked = false;
    let mut random_brick = 1;

    // Get difficulty
    let mut current_difficulty = 2;
    for points_text in textquery.iter_mut() {
        //println!("current_difficulty: {}", current_difficulty);
        current_difficulty = points_text.difficulty;
       
    }
    let random_brick_gen = generate_random_int(1..current_difficulty);


    for mut mouse_pos in query.iter_mut() {
        mouse_x = mouse_pos.x + (generate_random_int(-4..5)) as f32;
        mouse_y = mouse_pos.y;
        clicked = mouse_pos.clicked;
        random_brick = mouse_pos.next_random_brick;

        if clicked == true {
            // Reset
            mouse_pos.next_random_brick = random_brick_gen;
            mouse_pos.clicked = false;
        }
    }
    if clicked == true {
        let mut brick_next_id = 0;

        for (brick_transform, brick) in query_brick.iter_mut() {
            if brick_next_id <= brick.id {
                brick_next_id = brick.id + 1;
            }
        }

        //println!("brick_next_id: {}", brick_next_id);

        let random_brick_type = random_brick;
        let mut color_r = generate_random_int(0..100) as f32 / 100.0;
        let mut color_g = generate_random_int(0..100) as f32 / 100.0;
        let mut color_b = generate_random_int(0..100) as f32 / 100.0;

        if random_brick_type == 1 {
            color_r = 0.84;
            color_g = 0.10;
            color_b = 0.37;
        }

        if random_brick_type == 2 {
            color_r = 0.11;
            color_g = 0.53;
            color_b = 0.89;
        }

        if random_brick_type == 3 {
            color_r = 1.0;
            color_g = 0.75;
            color_b = 0.02;
        }

        if random_brick_type == 4 {
            color_r = 0.0;
            color_g = 0.30;
            color_b = 0.25;
        }

        if random_brick_type == 5 {
            color_r = 0.72;
            color_g = 0.02;
            color_b = 1.00;
        }

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
            Transform::from_translation(Vec3::new(mouse_x as f32, 100.0, 10.0))
                .with_scale(Vec2::splat(BRICK_SIZE).extend(1.)),
            Brick {
                id: brick_next_id,
                brick_type: random_brick_type,
                vel_x: 0.0,
                vel_y: 0.0,
                size: BRICK_SIZE,
                time_still: 0.0,
                time_still_move_x: 0.0,
                time_still_move_y: 0.0,
                to_delete: 0,
                game_lost: false,
                set_game_lost_text: false,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.5))),
            Transform::from_translation(Vec3::new(mouse_x, 100., 10.0))
                .with_scale(Vec2::splat(5.0).extend(1.0)),
            BrickCompare {
                id: brick_next_id,
                brick_type: random_brick_type,
                size: BRICK_SIZE,
            },
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }
}
