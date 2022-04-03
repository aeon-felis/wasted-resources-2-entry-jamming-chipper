use bevy::prelude::*;

use crate::global_types::AppState;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(setup_arena));
    }
}

fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut meterials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 2.0))),
        material: meterials.add(Color::BEIGE.into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..Default::default()
    });
}
