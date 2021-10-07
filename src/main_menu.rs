use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Align2, Area, Button, TextStyle, Window},
    EguiContext,
};

use crate::app_state::AppState;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainMenuState {
    Disabled,
    Idle,
    CustomGame,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(MainMenuState::Idle)
            .add_system_set(
                SystemSet::on_update(MainMenuState::Idle).with_system(main_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::CustomGame)
                    .with_system(custom_game_window_system.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(disable_main_menu_system.system()),
            );
    }
}

fn main_menu_system(
    egui: ResMut<EguiContext>,
    mut exit_event: EventWriter<AppExit>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
) {
    Area::new("Main Menu")
        .anchor(Align2::LEFT_CENTER, (15.0, 0.0))
        .show(egui.ctx(), |ui| {
            ui.add(
                Button::new("Play")
                    .text_style(TextStyle::Heading)
                    .enabled(false),
            );
            if ui
                .add(Button::new("Custom game").text_style(TextStyle::Heading))
                .clicked()
            {
                main_menu_state.set(MainMenuState::CustomGame).unwrap();
            }
            ui.add(
                Button::new("Characters")
                    .text_style(TextStyle::Heading)
                    .enabled(false),
            );
            ui.add(
                Button::new("Settings")
                    .text_style(TextStyle::Heading)
                    .enabled(false),
            );
            if ui
                .add(Button::new("Exit").text_style(TextStyle::Heading))
                .clicked()
            {
                exit_event.send(AppExit);
            }
        });
}

fn custom_game_window_system(
    egui: ResMut<EguiContext>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
    mut app_menu_state: ResMut<State<AppState>>,
) {
    let mut open = true;
    Window::new("Custom game")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .open(&mut open)
        .show(egui.ctx(), |ui| {
            if ui.button("Start").clicked() {
                app_menu_state.set(AppState::InGame).unwrap();
            }
        });

    if !open {
        main_menu_state.set(MainMenuState::Idle).unwrap();
    }
}

fn disable_main_menu_system(mut main_menu_state: ResMut<State<MainMenuState>>) {
    main_menu_state.set(MainMenuState::Disabled).unwrap();
}
