use bevy::prelude::*;

fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Hold 'Left' or 'Right' to change the line width",
        TextStyle {
            font: assert_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);
    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );
    gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);

    gizmos.circle_2d(Vec2::ZERO, 300., Color::NAVY).segments(64);

    gizmos.arc_2d(
        Vec2::ZERO,
        sin / 10.,
        std::f32::consts::PI / 2.,
        350.,
        Color::ORANGE_RED,
    );
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    if keyboard.pressed(KeyCode::Right) {
        config.line_width += 5. * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        config.line_width -= 5. * time.delta_seconds();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system, update_config))
        .run();
}
