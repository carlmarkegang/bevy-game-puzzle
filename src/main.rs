use std::slice::Windows;

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
    next_random_brick: i32
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
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 10.0))
            .with_scale(Vec2::splat(setupbrick::BRICK_SIZE).extend(1.)),
            MousePos {
                x: 0.0,
                y: 0.0,
                clicked: false,
                next_random_brick: generate_random_int(1..4)
            },
        setupcamera::PIXEL_PERFECT_LAYERS,
    ));

    // Background pixels
    for _i in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(
                generate_random_int(-90..90) as f32,
                generate_random_int(-90..90) as f32,
                2.,
            )
            .with_scale(Vec3::new(1.0, 1.0, 2.0)),
            Backgroundpixles,
            setupcamera::PIXEL_PERFECT_LAYERS,
        ));
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
    window: Query<&Window>
) {

    let window = window.single();

    let width = window.resolution.width();

    for ev in evr_cursor.read() {
        for (mut mouse_transform, mut mouse_pos) in query.iter_mut() {
            mouse_pos.x = (ev.position.x - width / 2.0).clamp(-50.0, 50.0); 
            mouse_pos.y = ev.position.y;

            mouse_transform.translation.x = mouse_pos.x;
            //println!("mouse x: {}", mouse_pos.x);

            let mut color_r = generate_random_int(0..100) as f32 / 100.0;
            let mut color_g = generate_random_int(0..100) as f32 / 100.0;
            let mut color_b = generate_random_int(0..100) as f32 / 100.0;
            if mouse_pos.next_random_brick == 1 {
                color_r = 1.0;
                color_g = 0.2;
                color_b = 0.2;
            }
    
            if mouse_pos.next_random_brick == 2 {
                color_r = 0.2;
                color_g = 1.0;
                color_b = 0.2;
            }
    
            if mouse_pos.next_random_brick == 3 {
                color_r = 0.2;
                color_g = 0.2;
                color_b = 1.0;
            }

            // set color
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        for (mut mouse_transform, mut mouse_pos) in query.iter_mut() {
            mouse_pos.clicked = true;
            mouse_pos.next_random_brick = generate_random_int(1..4);
        }
    }
}
