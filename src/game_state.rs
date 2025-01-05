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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_default() {
        assert_eq!(GameState::default(), GameState::MainMenu);
    }

    #[test]
    fn test_selected_tool() {
        let selected_tool = SelectedTool { tool: Tool::None };
        assert_eq!(selected_tool.tool, Tool::None);

        let selected_tool = SelectedTool { tool: Tool::LivingModule };
        assert_eq!(selected_tool.tool, Tool::LivingModule);
    }

    #[test]
    fn test_tool_equality() {
        assert_ne!(Tool::None, Tool::LivingModule);
        assert_ne!(Tool::OxygenPlant, Tool::PowerCable);
        assert_eq!(Tool::ScienceLab, Tool::ScienceLab);
    }
} 
