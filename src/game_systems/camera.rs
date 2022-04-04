use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let camera = PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.0, 30.0)
            .looking_at(Vec3::new(5.0, 0.0, 0.0), Vec3::Y),
        ..PerspectiveCameraBundle::new_3d()
    };
    commands.spawn_bundle(camera);

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 20.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}
