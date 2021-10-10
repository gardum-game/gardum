use crate::app_state::AppState;
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Align2, Area, Button, DragValue, Grid, TextStyle, Window},
    EguiContext,
};

const MARGIN: f32 = 20.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainMenuState {
    Disabled,
    Idle,
    CustomGame,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CustomGameWindowState>()
            .add_state(MainMenuState::Idle)
            .add_system_set(
                SystemSet::on_update(MainMenuState::Idle).with_system(main_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::CustomGame)
                    .with_system(custom_game_window_system.system())
                    .with_system(back_button_system.system()),
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
        .anchor(Align2::LEFT_CENTER, (MARGIN, 0.0))
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

#[derive(Default)]
struct CustomGameWindowState {
    search_text: String,
    map: String,
    teams_enabled: bool,
    teams_count: i8,
    slots_count: i8,
}

fn custom_game_window_system(
    egui: ResMut<EguiContext>,
    mut custom_game_state: Local<CustomGameWindowState>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Custom game")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut custom_game_state.search_text);
                if ui.button("Create").clicked() || ui.button("Connect").clicked() {
                    app_state.set(AppState::InGame).unwrap();
                }
                ui.group(|ui| {
                    Grid::new("Server Settings").show(ui, |ui| {
                        ui.label("Map:");
                        ui.text_edit_singleline(&mut custom_game_state.map);
                        ui.end_row();
                        ui.checkbox(&mut custom_game_state.teams_enabled, "Teams enabled");
                        ui.end_row();
                        ui.label("Teams count:");
                        ui.add(DragValue::new(&mut custom_game_state.teams_count));
                        ui.end_row();
                        ui.label("Slots count:");
                        ui.add(DragValue::new(&mut custom_game_state.slots_count));
                    });
                });
            })
        });
}

fn back_button_system(
    egui: ResMut<EguiContext>,
    input: Res<Input<KeyCode>>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
) {
    Area::new("Back area")
        .anchor(Align2::LEFT_BOTTOM, (MARGIN, -MARGIN))
        .show(egui.ctx(), |ui| {
            if input.just_pressed(KeyCode::Escape) || ui.button("Back").clicked() {
                main_menu_state.set(MainMenuState::Idle).unwrap();
            }
        });
}

fn disable_main_menu_system(mut main_menu_state: ResMut<State<MainMenuState>>) {
    main_menu_state.set(MainMenuState::Disabled).unwrap();
}
