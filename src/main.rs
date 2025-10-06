mod input;
mod pixel_world;
mod game;
mod player;
mod tree_chopping;
mod chunk_physics;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jungle Survival - Pixel Physics".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(input::InputPlugin)
        .add_plugins(pixel_world::PixelWorldPlugin)
        .add_plugins(game::GamePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(tree_chopping::TreeChoppingPlugin)
        .add_plugins(chunk_physics::ChunkPhysicsPlugin)
        .run();
}
