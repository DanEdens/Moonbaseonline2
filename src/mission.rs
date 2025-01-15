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
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let mission = Mission::load(mission_id.selected_mission);
    commands.insert_resource(mission.clone());
    
    // Reset camera position to center of map
    if let Ok(mut transform) = camera.get_single_mut() {
        let (width, height) = mission.map_size;
        transform.translation.x = (width as f32 * 10.0) / 2.0;
        transform.translation.y = (height as f32 * 10.0) / 2.0;
        transform.scale = Vec3::splat(crate::DEFAULT_ZOOM);
    }
    
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
            
            let position = Vec3::new(
                x as f32 * 10.0,
                y as f32 * 10.0,
                0.0
            );
            
            commands.spawn((
                MapTile { x, y, terrain },
                SpatialBundle {
                    transform: Transform::from_translation(position),
                    ..default()
                },
            ));
        }
    }
}

pub fn draw_map(
    mut gizmos: Gizmos,
    tiles: Query<(&MapTile, &Transform)>,
    mission: Res<Mission>,
) {
    // Draw tiles
    for (tile, transform) in tiles.iter() {
        let color = match tile.terrain {
            TerrainType::Flat => Color::GRAY,
            TerrainType::Rough => Color::DARK_GRAY,
            TerrainType::Crater => Color::BLACK,
            TerrainType::Mountain => Color::WHITE,
        };
        
        gizmos.rect_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
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
            if tiles.iter().any(|t| t.x == tile_x && t.y == tile_y) {
                // Highlight hovered tile
                gizmos.rect_2d(
                    Vec2::new(tile_x as f32 * 10.0, tile_y as f32 * 10.0),
                    0.0,
                    Vec2::new(10.0, 10.0),
                    Color::rgba(1.0, 1.0, 0.0, 0.3),
                );
                
                // TODO: Show terrain info in a tooltip instead of console logging
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

    #[test]
    fn test_mission_objectives() {
        let mission = Mission::load(1);
        assert!(mission.objectives.contains(&"Build a Living Module".to_string()));
        assert!(mission.objectives.contains(&"Connect power supply".to_string()));
        assert!(mission.objectives.contains(&"Establish oxygen production".to_string()));
        assert_eq!(mission.objectives.len(), 3);
    }

    #[test]
    fn test_map_generation() {
        let mut app = App::new();
        
        // Setup minimal app with required systems
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_mission);
        
        // Add required resource
        app.insert_resource(crate::ui::NewGameDialog {
            open: false,
            selected_mission: 1,
        });
        
        // Run systems
        app.update();
        
        // Verify mission resource was created
        let mission = app.world.get_resource::<Mission>().expect("Mission should be created");
        assert_eq!(mission.id, 1);
        
        // Verify map tiles were created
        let tile_count = app.world.query::<&MapTile>().iter(&app.world).count();
        assert_eq!(tile_count, 64 * 64); // Based on mission 1 map size
        
        // Verify terrain distribution
        let terrain_counts = app.world.query::<&MapTile>()
            .iter(&app.world)
            .fold((0, 0, 0, 0), |mut acc, tile| {
                match tile.terrain {
                    TerrainType::Flat => acc.0 += 1,
                    TerrainType::Rough => acc.1 += 1,
                    TerrainType::Crater => acc.2 += 1,
                    TerrainType::Mountain => acc.3 += 1,
                }
                acc
            });
            
        // Check rough percentages (allowing for some random variation)
        let total_tiles = (64 * 64) as f32;
        let flat_percentage = terrain_counts.0 as f32 / total_tiles;
        let rough_percentage = terrain_counts.1 as f32 / total_tiles;
        let crater_percentage = terrain_counts.2 as f32 / total_tiles;
        let mountain_percentage = terrain_counts.3 as f32 / total_tiles;
        
        assert!(flat_percentage > 0.6 && flat_percentage < 0.8); // ~70%
        assert!(rough_percentage > 0.1 && rough_percentage < 0.2); // ~15%
        assert!(crater_percentage > 0.05 && crater_percentage < 0.15); // ~10%
        assert!(mountain_percentage > 0.02 && mountain_percentage < 0.08); // ~5%
    }

    #[test]
    fn test_terrain_type_debug() {
        assert_eq!(format!("{:?}", TerrainType::Flat), "Flat");
        assert_eq!(format!("{:?}", TerrainType::Rough), "Rough");
        assert_eq!(format!("{:?}", TerrainType::Crater), "Crater");
        assert_eq!(format!("{:?}", TerrainType::Mountain), "Mountain");
    }
} 
