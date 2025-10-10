pub mod components;
pub mod switching;
pub mod usage;
pub mod hand;

use bevy::prelude::*;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<components::CurrentTool>()
            .init_resource::<components::GrabbedChunk>()
            .init_resource::<usage::ParticleSpawnTimer>()
            .init_resource::<usage::BlockBreakTimer>()
            .add_systems(Update, (
                switching::handle_tool_switching,
                usage::use_tool,
                hand::handle_hand_tool,
            ));
    }
}
