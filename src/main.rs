// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy::render::options::WgpuOptions;
use bevy::render::render_resource::WgpuFeatures;
use bevy_egui_kbgp::{KbgpNavBindings, KbgpPlugin, KbgpSettings};
use bevy_hanabi::HanabiPlugin;
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use bevy_tweening::TweeningPlugin;
use jamming_chipper::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 });
    app.insert_resource(ClearColor(Color::rgb(0.529, 0.808, 0.922)));
    app.insert_resource(WindowDescriptor {
        width: 800.,
        height: 600.,
        title: "Jamming Chipper".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GamePlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(TweeningPlugin);
    app.insert_resource({
        let mut options = WgpuOptions::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    });
    app.add_plugin(HanabiPlugin);
    app.add_plugin(bevy_egui_kbgp::bevy_egui::EguiPlugin);
    app.insert_resource(bevy_egui_kbgp::bevy_egui::EguiSettings { scale_factor: 2.0 });
    app.add_plugin(KbgpPlugin);
    app.insert_resource(KbgpSettings {
        allow_keyboard: true,
        allow_mouse_buttons: true,
        allow_mouse_wheel: true,
        allow_mouse_wheel_sideways: true,
        allow_gamepads: true,
        bindings: KbgpNavBindings::default().with_wasd_navigation(),
        disable_default_navigation: true,
        disable_default_activation: false,
        prevent_loss_of_focus: true,
        focus_on_mouse_movement: true,
    });
    app.run();
}
