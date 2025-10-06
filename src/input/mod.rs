use bevy::prelude::*;

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
) {
    // Movement input (Arrow keys)
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    game_input.movement = movement.normalize_or_zero();

    // Action inputs
    game_input.interact = mouse.just_pressed(MouseButton::Left);
    game_input.chop = mouse.just_pressed(MouseButton::Left);
    game_input.rotate_left = keyboard.pressed(KeyCode::KeyQ);
    game_input.rotate_right = keyboard.pressed(KeyCode::KeyE);
    game_input.place_object = mouse.just_pressed(MouseButton::Left);
    game_input.cancel = keyboard.just_pressed(KeyCode::Escape);

    // Mouse look
    let mut look_delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        look_delta += event.delta;
    }
    game_input.look_delta = look_delta;
}
