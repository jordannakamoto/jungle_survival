use bevy::prelude::*;
use crate::tools::components::{CurrentTool, Tool};

#[derive(Component)]
pub struct ToolIndicatorText;

pub fn setup_tool_indicator(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Hand"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(-330.0, -280.0, 1.0),
        ToolIndicatorText,
    ));
}

pub fn render_tool_indicator(
    current_tool: Res<CurrentTool>,
    mut gizmos: Gizmos,
) {
    // Draw tool indicator in bottom-left corner
    let position = Vec2::new(-380.0, -280.0);
    let size = Vec2::new(30.0, 30.0);

    // Background
    gizmos.rect_2d(
        Isometry2d::new(position, Rot2::IDENTITY),
        size,
        Color::srgb(0.2, 0.2, 0.2),
    );

    // Tool icon (simplified)
    match current_tool.tool {
        Tool::Hand => {
            // Draw hand icon
            gizmos.rect_2d(
                Isometry2d::new(position, Rot2::IDENTITY),
                Vec2::new(20.0, 20.0),
                Color::srgb(0.9, 0.8, 0.7),
            );
        }
        Tool::Axe => {
            // Draw axe icon
            gizmos.rect_2d(
                Isometry2d::new(position, Rot2::IDENTITY),
                Vec2::new(20.0, 20.0),
                Color::srgb(0.6, 0.3, 0.1),
            );
        }
        Tool::Shovel => {
            // Draw shovel icon
            gizmos.rect_2d(
                Isometry2d::new(position, Rot2::IDENTITY),
                Vec2::new(20.0, 20.0),
                Color::srgb(0.5, 0.5, 0.5),
            );
        }
    }
}

pub fn update_tool_indicator_text(
    current_tool: Res<CurrentTool>,
    mut query: Query<&mut Text2d, With<ToolIndicatorText>>,
) {
    if current_tool.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            **text = current_tool.tool.name().to_string();
        }
    }
}
