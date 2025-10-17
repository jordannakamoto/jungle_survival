use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugConfig::load())
            .init_resource::<DebugMenuVisible>()
            .add_systems(Update, (
                handle_debug_input,
                update_debug_render_context,
            ))
            .add_systems(bevy_egui::EguiPrimaryContextPass, draw_debug_menu);
    }
}

/// Persistent debug configuration
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct DebugConfig {
    /// Show physics debug rendering (Rapier colliders)
    pub show_physics_debug: bool,
    /// Show FPS counter
    pub show_fps: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            show_physics_debug: false,
            show_fps: false,
        }
    }
}

impl DebugConfig {
    const CONFIG_FILE: &'static str = "debug_config.json";

    /// Load config from file, or create default if it doesn't exist
    pub fn load() -> Self {
        if Path::new(Self::CONFIG_FILE).exists() {
            if let Ok(contents) = fs::read_to_string(Self::CONFIG_FILE) {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        Self::default()
    }

    /// Save config to file
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(Self::CONFIG_FILE, json);
        }
    }
}

/// Handle debug input toggles
fn handle_debug_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_visible: ResMut<DebugMenuVisible>,
) {
    // Backtick/tilde key to toggle debug menu
    if keyboard.just_pressed(KeyCode::Backquote) {
        menu_visible.0 = !menu_visible.0;
    }
}

#[derive(Resource)]
pub struct DebugMenuVisible(pub bool);

impl Default for DebugMenuVisible {
    fn default() -> Self {
        Self(false)
    }
}

/// Draw the debug menu using egui
fn draw_debug_menu(
    mut contexts: EguiContexts,
    menu_visible: Res<DebugMenuVisible>,
    mut debug_config: ResMut<DebugConfig>,
) {
    if !menu_visible.0 {
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("Developer Menu")
        .default_pos([10.0, 50.0])
        .default_width(250.0)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.heading("Debug Options");
            ui.add_space(10.0);

            if ui.checkbox(&mut debug_config.show_physics_debug, "Physics Debug Rendering").changed() {
                debug_config.save();
            }

            if ui.checkbox(&mut debug_config.show_fps, "Show FPS").changed() {
                debug_config.save();
            }

            ui.add_space(10.0);
            ui.separator();
            ui.label("Press ` to close");
        });
}

/// Update Rapier debug rendering context based on config
fn update_debug_render_context(
    debug_config: Res<DebugConfig>,
    mut debug_render_context: ResMut<bevy_rapier2d::render::DebugRenderContext>,
) {
    debug_render_context.enabled = debug_config.show_physics_debug;
}
