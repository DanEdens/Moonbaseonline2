use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::game_state::GameState;

mod ui;
mod game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Moonbase Online2".to_string(),
                ..default()
            }),
            ..default()
        }))
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
