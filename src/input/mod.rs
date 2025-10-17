use bevy::prelude::*;
use bevy_egui::EguiContexts;

/// Independent input controller that abstracts raw input into game actions
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameInput>()
            .add_systems(Update, update_game_input);
    }
}

/// High-level game input state - independent of specific keys/buttons
#[derive(Resource, Default)]
pub struct GameInput {
    pub movement: Vec2,
    pub interact: bool,
    pub chop: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub place_object: bool,
    pub cancel: bool,
    pub look_delta: Vec2,
}

fn update_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut game_input: ResMut<GameInput>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut contexts: EguiContexts,
) {
    // Check if egui wants the input
    let ctx = contexts.ctx_mut();
    let egui_wants_keyboard = ctx.as_ref().map(|c| c.wants_keyboard_input()).unwrap_or(false);
    let egui_wants_pointer = ctx.as_ref().map(|c| c.wants_pointer_input()).unwrap_or(false);

    // Movement input (WASD) - only if egui doesn't want keyboard
    let mut movement = Vec2::ZERO;
    if !egui_wants_keyboard {
        if keyboard.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }
    }
    game_input.movement = movement.normalize_or_zero();

    // Action inputs - only if egui doesn't want pointer/keyboard
    game_input.interact = !egui_wants_pointer && mouse.just_pressed(MouseButton::Left);
    game_input.chop = !egui_wants_pointer && mouse.just_pressed(MouseButton::Left);
    game_input.rotate_left = !egui_wants_keyboard && keyboard.pressed(KeyCode::KeyQ);
    game_input.rotate_right = !egui_wants_keyboard && keyboard.pressed(KeyCode::KeyE);
    game_input.place_object = !egui_wants_pointer && mouse.just_pressed(MouseButton::Left);
    game_input.cancel = !egui_wants_keyboard && keyboard.just_pressed(KeyCode::Escape);

    // Mouse look - only if egui doesn't want pointer
    let mut look_delta = Vec2::ZERO;
    if !egui_wants_pointer {
        for event in mouse_motion.read() {
            look_delta += event.delta;
        }
    }
    game_input.look_delta = look_delta;
}
