use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::MainMenu);
    }
}
