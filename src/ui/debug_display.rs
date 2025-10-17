use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Component)]
pub struct DebugText;

pub fn setup_debug_display(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        DebugText,
    ));
}

pub fn update_debug_display(
    mut query: Query<&mut Text, With<DebugText>>,
    debug_config: Res<crate::debug::DebugConfig>,
    menu_visible: Res<crate::debug::DebugMenuVisible>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };

    let mut lines = Vec::new();

    // Show FPS if enabled
    if debug_config.show_fps {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                lines.push(format!("FPS: {:.1}", value));
            }
        }
    }

    // Show active debug modes
    let mut active_modes = Vec::new();
    if debug_config.show_physics_debug {
        active_modes.push("Physics");
    }

    if !active_modes.is_empty() {
        lines.push(format!("Debug: {}", active_modes.join(", ")));
    }

    // Update text
    **text = lines.join("\n");
}
