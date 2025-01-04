use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod ui;
mod game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            ui::top_menu_bar,
            ui::side_toolbar,
            ui::handle_tool_selection,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
} 
