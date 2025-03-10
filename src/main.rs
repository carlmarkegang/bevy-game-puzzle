use bevy::prelude::*;
use rand::Rng;
mod setupbrick;
mod setupcamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Startup,
            (
                setupcamera::setup_camera,
                setup_main,
                setupbrick::setup_brick,
            ),
        )
        .add_systems(Update, (setupcamera::fit_canvas, cursor_events))
        .add_systems(
            FixedUpdate,
            (
                setupbrick::set_pos_compare_brick,
                setupbrick::time_still_check,
                setupbrick::collision_check_brick,
                setupbrick::brick_movements,
                setupbrick::check_touching,
                setupbrick::check_touching,
                setupbrick::delete_touching,
                setupbrick::spawn_brick,
                backgroundpixles_movement,
                update_point_text,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Backgroundpixles;

#[derive(Component)]
struct MousePos {
    x: f32,
    y: f32,
    clicked: bool,
    next_random_brick: i32,
    time_from_clicked: f32,
}

#[derive(Component)]
struct PointsText {
    points: i32,
    difficulty: i32
}

fn setup_main(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    /*
    commands.spawn(MousePos {
        x: 0.0,
        y: 0.0,
        clicked: false,
    });
     */

     commands.spawn((
        Text::new("Points: 0"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
        PointsText { 
            points: 0,
            difficulty: 2 
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    let mut gen_next_random_brick = 1;
    let mut color_r = generate_random_int(0..100) as f32 / 100.0;
    let mut color_g = generate_random_int(0..100) as f32 / 100.0;
    let mut color_b = generate_random_int(0..100) as f32 / 100.0;
    if gen_next_random_brick == 1 {
        color_r = 0.84;
        color_g = 0.10;
        color_b = 0.37;
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
            .with_scale(Vec2::splat(setupbrick::BRICK_SIZE).extend(1.)),
        MousePos {
            x: 0.0,
            y: 0.0,
            clicked: false,
            next_random_brick: gen_next_random_brick,
            time_from_clicked: 0.0,
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 2;
    if gen_next_random_brick == 2 {
        color_r = 0.11;
        color_g = 0.53;
        color_b = 0.89;
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
            .with_scale(Vec2::splat(setupbrick::BRICK_SIZE).extend(1.)),
        MousePos {
            x: 0.0,
            y: 0.0,
            clicked: false,
            next_random_brick: gen_next_random_brick,
            time_from_clicked: 0.0,
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 3;
    if gen_next_random_brick == 3 {
        color_r = 1.0;
        color_g = 0.75;
        color_b = 0.02;
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
            .with_scale(Vec2::splat(setupbrick::BRICK_SIZE).extend(1.)),
        MousePos {
            x: 0.0,
            y: 0.0,
            clicked: false,
            next_random_brick: gen_next_random_brick,
            time_from_clicked: 0.0,
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 4;
    if gen_next_random_brick == 4 {
        color_r = 0.0;
        color_g = 0.30;
        color_b = 0.25;
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
            .with_scale(Vec2::splat(setupbrick::BRICK_SIZE).extend(1.)),
        MousePos {
            x: 0.0,
            y: 0.0,
            clicked: false,
            next_random_brick: gen_next_random_brick,
            time_from_clicked: 0.0,
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 5;
    if gen_next_random_brick == 5 {
        color_r = 0.72;
        color_g = 0.02;
        color_b = 1.00;
    }

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(color_r, color_g, color_b))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
            .with_scale(Vec2::splat(setupbrick::BRICK_SIZE).extend(1.)),
        MousePos {
            x: 0.0,
            y: 0.0,
            clicked: false,
            next_random_brick: gen_next_random_brick,
            time_from_clicked: 0.0,
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    // Background pixels

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.2))),
        Transform::from_xyz(0 as f32, 0 as f32, 2.).with_scale(Vec3::new(120.0, 185.0, 0.0)),
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.4, 0.4))),
        Transform::from_xyz(0 as f32, 80. as f32, 20.).with_scale(Vec3::new(120.0, 1.0, 20.0)),
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    for _i in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(
                generate_random_int(-50..50) as f32,
                generate_random_int(-100..100) as f32,
                5.,
            )
            .with_scale(Vec3::new(1.0, 1.0, 3.0)),
            Backgroundpixles,
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }


}

fn update_point_text(mut textquery: Query<(&mut Text, &mut PointsText)>) {
    for (mut span, mut points_text) in textquery.iter_mut() {
        let value = format!("Points: {}", points_text.points);
        **span = format!("{value}");

        if points_text.points > 2000 {
            points_text.difficulty = 3;
        }
       
        if points_text.points > 5000 {
            points_text.difficulty = 4;
        }
        if points_text.points > 10000 {
            points_text.difficulty = 5;
        }
        if points_text.points > 20000 {
            points_text.difficulty = 6;
        }

    }
}

fn backgroundpixles_movement(mut transforms: Query<&mut Transform, With<Backgroundpixles>>) {
    for mut transform in &mut transforms {
        if generate_random_int(0..50) == 0 {
            transform.translation.x = generate_random_int(-55..55) as f32;
            transform.translation.y = generate_random_int(-90..90) as f32;
        }
    }
}

fn generate_random_int(maxmin: std::ops::Range<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let generated_float = rng.gen_range(maxmin);
    generated_float
}

fn cursor_events(
    mut evr_cursor: EventReader<CursorMoved>,
    mut query: Query<(&mut Transform, &mut MousePos)>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let width = window.resolution.width();

    for ev in evr_cursor.read() {
        for (mut mouse_transform, mut mouse_pos) in query.iter_mut() {
            mouse_pos.x = (ev.position.x - width / 2.0).clamp(-50.0, 50.0);
            mouse_pos.y = ev.position.y;

            mouse_transform.translation.x = mouse_pos.x;
            //println!("mouse x: {}", mouse_pos.x);
        }
    }

    for (mut mouse_transform, mut mouse_pos) in query.iter_mut() {
        mouse_transform.translation.y = 200.0;
        mouse_transform.translation.z = 20.0;
        mouse_pos.time_from_clicked += 1.0;
    }

    let mut i = 1;
    for (mut mouse_transform, mouse_pos) in query.iter_mut() {
        if i == mouse_pos.next_random_brick {
            mouse_transform.translation.z = 50.;
            if mouse_pos.time_from_clicked > 300. {
                mouse_transform.translation.y = 100.0;
            }
        }
        i += 1;
    }

    if buttons.just_pressed(MouseButton::Left) {
        for (mouse_transform, mut mouse_pos) in query.iter_mut() {
            if mouse_pos.time_from_clicked > 300. {
                mouse_pos.clicked = true;
                mouse_pos.time_from_clicked = 0.0;
            }
        }
    }
}
