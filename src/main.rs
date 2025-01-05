use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};
use crate::game_state::GameState;

mod ui;
mod game_state;
mod mission;

const CAMERA_SPEED: f32 = 500.0;
const ZOOM_SPEED: f32 = 1.0;
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
    // Camera with default zoom
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(320.0, 320.0, 999.9),
            scale: Vec3::splat(DEFAULT_ZOOM),
            ..default()
        },
        ..default()
    });
}

fn camera_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera2d>>,
) {
    let (mut camera_transform, projection) = camera.single_mut();
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
    
    for event in mouse_wheel.iter() {
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / 100.0,
        };
        
        let zoom_factor = 1.0 - scroll_amount * ZOOM_SPEED;
        let new_scale = (transform.scale.x * zoom_factor)
            .clamp(MIN_ZOOM, MAX_ZOOM);
            
        transform.scale = Vec3::splat(new_scale);
    }
} 
