use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::game_state::{GameState, Tool, SelectedTool};

const TOOLBAR_WIDTH: f32 = 60.0;

pub fn top_menu_bar(
    mut contexts: EguiContexts,
) {
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Game").clicked() {
                    // Handle new game
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
