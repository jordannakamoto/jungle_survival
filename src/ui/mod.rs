pub mod tool_indicator;
pub mod debug_display;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, (
                tool_indicator::setup_tool_indicator,
                debug_display::setup_debug_display,
            ))
            .add_systems(Update, (
                tool_indicator::render_tool_indicator,
                tool_indicator::update_tool_indicator_text,
                debug_display::update_debug_display,
            ));
    }
}
