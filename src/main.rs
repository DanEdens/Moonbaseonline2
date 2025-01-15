use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};
use crate::game_state::GameState;

mod ui;
mod game_state;
mod mission;

const CAMERA_SPEED: f32 = 500.0;
const ZOOM_SPEED: f32 = 0.5;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;
const DEFAULT_ZOOM: f32 = 0.5;

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
        .init_resource::<ui::NewGameDialog>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            ui::top_menu_bar,
            ui::side_toolbar,
            ui::handle_tool_selection,
            ui::new_game_dialog,
        ))
        .add_systems(OnExit(GameState::MainMenu), cleanup_map)
        .add_systems(OnEnter(GameState::Playing), mission::setup_mission)
        .add_systems(Update, (
            mission::draw_map,
            camera_movement,
            camera_zoom,
            mission::handle_tile_hover,
        ).chain().run_if(in_state(GameState::Playing)))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera with default zoom - position it at the center of a typical map size
    commands.spawn(Camera2dBundle {
        transform: Transform {
            // Position for a 64x64 map
            translation: Vec3::new(320.0, 320.0, 999.9), // 64 * 10 / 2 = 320
            scale: Vec3::splat(DEFAULT_ZOOM),
            ..default()
        },
        ..default()
    });
}

fn camera_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = camera.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    if direction != Vec3::ZERO {
        // Adjust movement speed based on zoom level
        let zoom_factor = 1.0 / camera_transform.scale.x;
        camera_transform.translation += direction.normalize() * CAMERA_SPEED * time.delta_seconds() * zoom_factor;
    }
}

fn camera_zoom(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = query.single_mut();
    let mut total_scroll = 0.0;
    
    // Accumulate all scroll events
    for event in mouse_wheel.read() {
        total_scroll += match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / 100.0,
        };
    }
    
    if total_scroll != 0.0 {
        // Apply zoom based on total scroll amount
        let zoom_factor = if total_scroll > 0.0 {
            1.0 + ZOOM_SPEED  // Zoom out
        } else {
            1.0 / (1.0 + ZOOM_SPEED)  // Zoom in
        };
        
        let new_scale = (transform.scale.x * zoom_factor)
            .clamp(MIN_ZOOM, MAX_ZOOM);
            
        transform.scale = Vec3::splat(new_scale);
    }
}

fn cleanup_map(
    mut commands: Commands,
    map_tiles: Query<Entity, With<mission::MapTile>>,
) {
    // Remove all existing map tiles
    for entity in map_tiles.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::mouse::MouseScrollUnit;
    use std::time::Duration;

    #[test]
    fn test_camera_movement() {
        let mut app = App::new();
        
        // Setup minimal app with required systems
        app.add_plugins(MinimalPlugins)
            .add_systems(Update, camera_movement);
        
        // Spawn camera
        app.world.spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.9),
            ..default()
        });

        // Add time resource with a fixed delta
        app.world.insert_resource(Time::new_with(Duration::from_secs_f32(1.0/60.0)));

        // Test movement in each direction
        let cases = vec![
            (KeyCode::W, Vec3::new(0.0, 1.0, 0.0)),
            (KeyCode::S, Vec3::new(0.0, -1.0, 0.0)),
            (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
            (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
        ];

        for (key, expected_dir) in cases {
            // Reset camera position
            let mut transform = app.world.query::<&mut Transform>().single_mut(&mut app.world);
            transform.translation = Vec3::new(0.0, 0.0, 999.9);

            let mut input = Input::<KeyCode>::default();
            input.press(key);
            app.world.insert_resource(input);
            
            // Run systems multiple times to accumulate movement
            for _ in 0..3 {
                app.update();
            }
            
            // Check camera moved in correct direction
            let transform = app.world.query::<&Transform>().single(&app.world);
            assert!(transform.translation.dot(expected_dir) > 0.0, 
                "Camera should move in direction {:?} for key {:?}", expected_dir, key);
        }
    }

    #[test]
    fn test_camera_zoom() {
        let mut app = App::new();
        
        // Setup minimal app with required systems
        app.add_plugins(MinimalPlugins)
            .add_systems(Update, camera_zoom);
        
        // Spawn camera with default zoom
        app.world.spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.9)
                .with_scale(Vec3::splat(DEFAULT_ZOOM)),
            ..default()
        });

        let window_entity = app.world.spawn_empty().id();

        // Test zoom in with multiple scroll events
        let mut events = Events::<MouseWheel>::default();
        for _ in 0..3 {  // Send multiple scroll events
            events.send(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: 0.0,
                y: -2.0,  // Scroll down to zoom in
                window: window_entity,
            });
        }
        app.world.insert_resource(events);
        app.update();

        let transform = app.world.query::<&Transform>().single(&app.world);
        let scale_after_zoom_in = transform.scale.x;
        assert!(scale_after_zoom_in < DEFAULT_ZOOM,
            "Multiple zoom in events should decrease scale - current: {}, default: {}",
            scale_after_zoom_in, DEFAULT_ZOOM);

        // Test zoom out with multiple scroll events
        let mut transform = app.world.query::<&mut Transform>().single_mut(&mut app.world);
        transform.scale = Vec3::splat(DEFAULT_ZOOM);

        let mut events = Events::<MouseWheel>::default();
        for _ in 0..3 {  // Send multiple scroll events
            events.send(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: 0.0,
                y: 2.0,  // Scroll up to zoom out
                window: window_entity,
            });
        }
        app.world.insert_resource(events);
        app.update();

        let transform = app.world.query::<&Transform>().single(&app.world);
        let scale_after_zoom_out = transform.scale.x;
        assert!(scale_after_zoom_out > DEFAULT_ZOOM,
            "Multiple zoom out events should increase scale - current: {}, default: {}",
            scale_after_zoom_out, DEFAULT_ZOOM);

        // Test minimum zoom limit
        let mut transform = app.world.query::<&mut Transform>().single_mut(&mut app.world);
        transform.scale = Vec3::splat(MIN_ZOOM * 1.1);  // Just above minimum

        let mut events = Events::<MouseWheel>::default();
        for _ in 0..5 {  // Send multiple scroll events
            events.send(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: 0.0,
                y: -2.0,  // Try to zoom in past minimum
                window: window_entity,
            });
        }
        app.world.insert_resource(events);
        app.update();

        let transform = app.world.query::<&Transform>().single(&app.world);
        assert!(transform.scale.x >= MIN_ZOOM,
            "Scale should not go below minimum - current: {}, min: {}",
            transform.scale.x, MIN_ZOOM);

        // Test maximum zoom limit
        let mut transform = app.world.query::<&mut Transform>().single_mut(&mut app.world);
        transform.scale = Vec3::splat(MAX_ZOOM * 0.9);  // Just below maximum

        let mut events = Events::<MouseWheel>::default();
        for _ in 0..5 {  // Send multiple scroll events
            events.send(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: 0.0,
                y: 2.0,  // Try to zoom out past maximum
                window: window_entity,
            });
        }
        app.world.insert_resource(events);
        app.update();

        let transform = app.world.query::<&Transform>().single(&app.world);
        assert!(transform.scale.x <= MAX_ZOOM,
            "Scale should not exceed maximum - current: {}, max: {}",
            transform.scale.x, MAX_ZOOM);
    }
} 
