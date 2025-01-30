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
        .add_systems(
            Update,
            (
                setupcamera::fit_canvas,
                cursor_events,
                setupbrick::set_pos_compare_brick,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                setupbrick::time_still_check,
                setupbrick::collision_check_brick,
                setupbrick::set_time_compare_brick,
                setupbrick::brick_movements,
                setupbrick::check_touching,
                setupbrick::check_touching,
                setupbrick::delete_touching,
                setupbrick::spawn_brick,
                backgroundpixles_movement,
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

    let mut gen_next_random_brick = 1;
    let mut color_r = generate_random_int(0..100) as f32 / 100.0;
    let mut color_g = generate_random_int(0..100) as f32 / 100.0;
    let mut color_b = generate_random_int(0..100) as f32 / 100.0;
    if gen_next_random_brick == 1 {
        color_r = 1.0;
        color_g = 0.2;
        color_b = 0.2;
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
            time_from_clicked: 0.0
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 2;
    if gen_next_random_brick == 2 {
        color_r = 0.2;
        color_g = 1.0;
        color_b = 0.2;
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
            time_from_clicked: 0.0
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 3;
    if gen_next_random_brick == 3 {
        color_r = 0.2;
        color_g = 0.2;
        color_b = 1.0;
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
            time_from_clicked: 0.0
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    gen_next_random_brick = 4;
    if gen_next_random_brick == 4 {
        color_r = 1.0;
        color_g = 1.0;
        color_b = 0.2;
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
            time_from_clicked: 0.0
        },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));


    // Background pixels
    for _i in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(
                generate_random_int(-50..50) as f32,
                generate_random_int(-100..100) as f32,
                2.,
            )
            .with_scale(Vec3::new(1.0, 1.0, 2.0)),
            Backgroundpixles,
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
    }
}

fn backgroundpixles_movement(
    mut transforms: Query<&mut Transform, With<Backgroundpixles>>,
) {
    for mut transform in &mut transforms {
        if generate_random_int(0..10) == 0 {
                transform.translation.x = generate_random_int(-50..50) as f32;
                transform.translation.y = generate_random_int(-100..100) as f32;
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
    for (mut mouse_transform, mut mouse_pos) in query.iter_mut() {

        if i == mouse_pos.next_random_brick {
            mouse_transform.translation.z = 50.;
            if mouse_pos.time_from_clicked > 400.{
                mouse_transform.translation.y = 100.0;
            }  
        }
        i += 1;
    }
    
    if buttons.just_pressed(MouseButton::Left) {
        for (mut mouse_transform, mut mouse_pos) in query.iter_mut() {
            if mouse_pos.time_from_clicked > 400.{
                mouse_pos.clicked = true;
                mouse_pos.time_from_clicked = 0.0;
            }
        }
        
    }
}
