use bevy::prelude::*;
use rand::Rng;
mod setupcamera;
mod setupbrick;

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
                setupbrick::time_still_check,
                setupbrick::collision_check_brick,
                setupbrick::set_pos_compare_brick,
                setupbrick::brick_movements,
                setupbrick::check_touching,
                setupbrick::check_touching,
                setupbrick::delete_touching
                
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
}

fn setup_main(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MousePos { x: 0.0, y: 0.0 });

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

fn cursor_events(mut evr_cursor: EventReader<CursorMoved>, mut query: Query<&mut MousePos>) {
    for ev in evr_cursor.read() {
        // Log the cursor position
        /*
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.window
        );
         */

        for mut mouse_pos in query.iter_mut() {
            mouse_pos.x = ev.position.x;
            mouse_pos.y = ev.position.y;
        }
    }
}
