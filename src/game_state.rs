use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}

#[derive(Resource)]
pub struct SelectedTool {
    pub tool: Tool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    None,
    LivingModule,
    OxygenPlant,
    PowerCable,
    ScienceLab,
    Telescope,
    HeliumMine,
} 
