use bevy::prelude::*;
use bevy_egui_kbgp::bevy_egui::EguiContext;
use bevy_egui_kbgp::egui;
use bevy_egui_kbgp::prelude::*;

use crate::global_types::{AppState, MenuState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Main)).with_system(main_menu),
        );
    }
}

fn menu_layout(egui_context: &egui::Context, dlg: impl FnOnce(&mut egui::Ui)) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_context, |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center);
            ui.with_layout(layout, |ui| {
                dlg(ui);
            });
        });
}

fn main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui
            .button("Start")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::ClearLevelAndThenLoad).unwrap();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().clicked() {
            exit.send(bevy::app::AppExit);
        }
    });
}
