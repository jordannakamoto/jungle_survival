mod constants;
mod input;
mod world;
mod player;
mod physics;
mod tools;
mod ui;
mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_egui::EguiPlugin;
use constants::{WORLD_PIXEL_WIDTH, WORLD_PIXEL_HEIGHT};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jungle Survival - Pixel Physics".to_string(),
                resolution: (WORLD_PIXEL_WIDTH as f32, WORLD_PIXEL_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(input::InputPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(tools::ToolsPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
