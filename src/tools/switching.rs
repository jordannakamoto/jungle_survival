use bevy::prelude::*;
use super::components::CurrentTool;

pub fn handle_tool_switching(
    mut current_tool: ResMut<CurrentTool>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        current_tool.tool = current_tool.tool.next();
        info!("Switched to: {}", current_tool.tool.name());
    }
}
