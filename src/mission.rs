use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

#[derive(Resource, Clone)]
pub struct Mission {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub map_size: (u32, u32),
    pub starting_funds: f32,
}

impl Mission {
    pub fn load(mission_id: usize) -> Self {
        // For now, return hardcoded data for Mission 1
        match mission_id {
            1 => Self {
                id: 1,
                name: "First Steps".to_string(),
                description: "Establish your first lunar base with basic facilities.".to_string(),
                objectives: vec![
                    "Build a Living Module".to_string(),
                    "Connect power supply".to_string(),
                    "Establish oxygen production".to_string(),
                ],
                map_size: (64, 64),
                starting_funds: 1000000.0,
            },
            _ => panic!("Mission {} not implemented yet", mission_id),
        }
    }
}

#[derive(Component)]
pub struct MapTile {
    pub x: u32,
    pub y: u32,
    pub terrain: TerrainType,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TerrainType {
    Flat,
    Rough,
    Crater,
    Mountain,
}

impl TerrainType {
    pub fn description(&self) -> &'static str {
        match self {
            TerrainType::Flat => "Flat terrain - Ideal for construction",
            TerrainType::Rough => "Rough terrain - Requires leveling",
            TerrainType::Crater => "Crater - Difficult to build on",
            TerrainType::Mountain => "Mountain - Unsuitable for construction",
        }
    }
}

pub fn setup_mission(
    mut commands: Commands,
    mission_id: Res<crate::ui::NewGameDialog>,
) {
    let mission = Mission::load(mission_id.selected_mission);
    commands.insert_resource(mission.clone());
    
    // Generate map
    let mut rng = rand::thread_rng();
    let (width, height) = mission.map_size;
    
    for y in 0..height {
        for x in 0..width {
            let terrain = match rng.gen_range(0..100) {
                0..=70 => TerrainType::Flat,    // 70% chance
                71..=85 => TerrainType::Rough,  // 15% chance
                86..=95 => TerrainType::Crater, // 10% chance
                _ => TerrainType::Mountain,     // 5% chance
            };
            
            commands.spawn((
                MapTile { x, y, terrain },
                SpatialBundle::default(), // This will be used later for positioning
            ));
        }
    }
}

pub fn draw_map(
    mut gizmos: Gizmos,
    tiles: Query<(&MapTile, &GlobalTransform)>,
    mission: Res<Mission>,
) {
    // Draw tiles
    for (tile, _transform) in tiles.iter() {
        let color = match tile.terrain {
            TerrainType::Flat => Color::GRAY,
            TerrainType::Rough => Color::DARK_GRAY,
            TerrainType::Crater => Color::BLACK,
            TerrainType::Mountain => Color::WHITE,
        };
        
        let pos = Vec2::new(tile.x as f32 * 10.0, tile.y as f32 * 10.0);
        gizmos.rect_2d(
            pos,
            0.0,
            Vec2::new(9.0, 9.0),
            color,
        );
    }

    // Draw grid
    let (width, height) = mission.map_size;
    for x in 0..=width {
        gizmos.line_2d(
            Vec2::new(x as f32 * 10.0, 0.0),
            Vec2::new(x as f32 * 10.0, height as f32 * 10.0),
            Color::rgba(1.0, 1.0, 1.0, 0.2),
        );
    }
    for y in 0..=height {
        gizmos.line_2d(
            Vec2::new(0.0, y as f32 * 10.0),
            Vec2::new(width as f32 * 10.0, y as f32 * 10.0),
            Color::rgba(1.0, 1.0, 1.0, 0.2),
        );
    }
}

pub fn handle_tile_hover(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<&MapTile>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera.single();
    let window = windows.single();
    
    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            let tile_x = (world_pos.x / 10.0).floor() as u32;
            let tile_y = (world_pos.y / 10.0).floor() as u32;
            
            // Find the tile under cursor
            if let Some(tile) = tiles.iter().find(|t| t.x == tile_x && t.y == tile_y) {
                // Highlight hovered tile
                gizmos.rect_2d(
                    Vec2::new(tile_x as f32 * 10.0, tile_y as f32 * 10.0),
                    0.0,
                    Vec2::new(10.0, 10.0),
                    Color::rgba(1.0, 1.0, 0.0, 0.3),
                );
                
                // TODO: Show terrain info in a tooltip
                // For now we'll print to console
                println!("Terrain at ({}, {}): {}", tile_x, tile_y, tile.terrain.description());
            }
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_load() {
        let mission = Mission::load(1);
        assert_eq!(mission.id, 1);
        assert_eq!(mission.name, "First Steps");
        assert_eq!(mission.map_size, (64, 64));
        assert!(mission.starting_funds > 0.0);
        assert!(!mission.objectives.is_empty());
    }

    #[test]
    #[should_panic(expected = "Mission 999 not implemented yet")]
    fn test_mission_load_invalid() {
        Mission::load(999);
    }

    #[test]
    fn test_terrain_descriptions() {
        assert_eq!(TerrainType::Flat.description(), "Flat terrain - Ideal for construction");
        assert_eq!(TerrainType::Rough.description(), "Rough terrain - Requires leveling");
        assert_eq!(TerrainType::Crater.description(), "Crater - Difficult to build on");
        assert_eq!(TerrainType::Mountain.description(), "Mountain - Unsuitable for construction");
    }

    #[test]
    fn test_map_tile_creation() {
        let tile = MapTile {
            x: 10,
            y: 20,
            terrain: TerrainType::Flat,
        };
        
        assert_eq!(tile.x, 10);
        assert_eq!(tile.y, 20);
        assert_eq!(tile.terrain, TerrainType::Flat);
    }
} 
