pub mod tool_indicator;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tool_indicator::setup_tool_indicator)
            .add_systems(Update, (
                tool_indicator::render_tool_indicator,
                tool_indicator::update_tool_indicator_text,
            ));
    }
}
