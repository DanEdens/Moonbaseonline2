use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::game_state::{Tool, SelectedTool, GameState};

const TOOLBAR_WIDTH: f32 = 60.0;

#[derive(Resource)]
pub struct NewGameDialog {
    pub open: bool,
    pub selected_mission: usize,
}

impl Default for NewGameDialog {
    fn default() -> Self {
        Self {
            open: false,
            selected_mission: 1,
        }
    }
}

pub fn top_menu_bar(
    mut contexts: EguiContexts,
    mut new_game_dialog: ResMut<NewGameDialog>,
) {
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Game").clicked() {
                    new_game_dialog.open = true;
                }
                if ui.button("Save").clicked() {
                    // Handle save
                }
                if ui.button("Load").clicked() {
                    // Handle load
                }
                if ui.button("Exit").clicked() {
                    // Handle exit
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("Map View").clicked() {}
                if ui.button("Financial Report").clicked() {}
            });

            ui.menu_button("Markets", |ui| {
                if ui.button("Trade Center").clicked() {}
                if ui.button("Research").clicked() {}
            });

            ui.menu_button("Operations", |ui| {
                if ui.button("Mission Control").clicked() {}
                if ui.button("Staff Management").clicked() {}
            });
        });
    });
}

pub fn new_game_dialog(
    mut contexts: EguiContexts,
    mut new_game_dialog: ResMut<NewGameDialog>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if new_game_dialog.open {
        egui::Window::new("New Game")
            .collapsible(false)
            .resizable(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Select Mission");
                ui.add_space(8.0);
                
                for mission in 1..=6 {
                    let text = match mission {
                        1 => "Mission 1: First Steps - Basic Base Setup",
                        2 => "Mission 2: Power Grid - Energy Management",
                        3 => "Mission 3: Life Support - Oxygen Systems",
                        4 => "Mission 4: Research - Science Operations",
                        5 => "Mission 5: Mining - Resource Extraction",
                        6 => "Mission 6: Full Operations - Complete Base",
                        _ => unreachable!()
                    };
                    
                    if ui.radio_value(&mut new_game_dialog.selected_mission, mission, text).clicked() {
                        // Mission selected
                    }
                }
                
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button("Start Mission").clicked() {
                        new_game_dialog.open = false;
                        next_state.set(GameState::Playing);
                    }
                    if ui.button("Cancel").clicked() {
                        new_game_dialog.open = false;
                    }
                });
            });
    }
}

pub fn side_toolbar(
    mut contexts: EguiContexts,
    mut commands: Commands,
) {
    egui::SidePanel::left("toolbar")
        .exact_width(TOOLBAR_WIDTH)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("🏠").clicked() {
                    commands.insert_resource(SelectedTool { tool: Tool::LivingModule });
                }
                if ui.button("🌬️").clicked() {
                    commands.insert_resource(SelectedTool { tool: Tool::OxygenPlant });
                }
                if ui.button("⚡").clicked() {
                    commands.insert_resource(SelectedTool { tool: Tool::PowerCable });
                }
                if ui.button("🔬").clicked() {
                    commands.insert_resource(SelectedTool { tool: Tool::ScienceLab });
                }
                if ui.button("🔭").clicked() {
                    commands.insert_resource(SelectedTool { tool: Tool::Telescope });
                }
                if ui.button("⛏️").clicked() {
                    commands.insert_resource(SelectedTool { tool: Tool::HeliumMine });
                }
            });
        });
}

pub fn handle_tool_selection(
    tool: Option<Res<SelectedTool>>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
) {
    if let Some(tool) = tool {
        if buttons.just_pressed(MouseButton::Left) {
            if let Ok(window) = windows.get_single() {
                if let Some(position) = window.cursor_position() {
                    // Handle tool placement at cursor position
                    println!("Placing {:?} at {:?}", tool.tool, position);
                }
            }
        }
    }
} 
