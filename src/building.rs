use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::input::GameInput;
use crate::interaction::{Log, Highlighted};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pickup_log_system, rotate_held_log_system, place_log_system).chain());
    }
}

#[derive(Component)]
pub struct HeldObject {
    pub offset: Vec2,
}

fn pickup_log_system(
    mut commands: Commands,
    game_input: Res<GameInput>,
    log_query: Query<(Entity, &Highlighted), (With<Log>, Without<HeldObject>)>,
    held_query: Query<Entity, With<HeldObject>>,
) {
    // Only pick up if not already holding something
    if !held_query.is_empty() {
        return;
    }

    if !game_input.interact {
        return;
    }

    // Pick up highlighted log
    for (entity, _) in log_query.iter() {
        commands
            .entity(entity)
            .remove::<RigidBody>()
            .remove::<Velocity>()
            .insert(HeldObject {
                offset: Vec2::new(40.0, 20.0),
            });
        break; // Only pick up one
    }
}

fn rotate_held_log_system(
    time: Res<Time>,
    game_input: Res<GameInput>,
    mut held_query: Query<&mut Transform, With<HeldObject>>,
) {
    let rotation_speed = 3.0;

    for mut transform in held_query.iter_mut() {
        let mut rotation_delta = 0.0;

        if game_input.rotate_left {
            rotation_delta += rotation_speed * time.delta_secs();
        }
        if game_input.rotate_right {
            rotation_delta -= rotation_speed * time.delta_secs();
        }

        if rotation_delta.abs() > 0.0 {
            let current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
            transform.rotation = Quat::from_rotation_z(current_rotation + rotation_delta);
        }
    }
}

fn place_log_system(
    mut commands: Commands,
    game_input: Res<GameInput>,
    player_query: Query<&Transform, Without<HeldObject>>,
    mut held_query: Query<(Entity, &mut Transform, &HeldObject), With<Log>>,
) {
    // Update held object position to follow player
    if let Ok(player_transform) = player_query.single() {
        for (_entity, mut transform, held) in held_query.iter_mut() {
            transform.translation.x = player_transform.translation.x + held.offset.x;
            transform.translation.y = player_transform.translation.y + held.offset.y;
        }
    }

    // Place log when left mouse clicked
    if game_input.place_object {
        for (entity, _transform, _) in held_query.iter() {
            commands
                .entity(entity)
                .remove::<HeldObject>()
                .insert(RigidBody::Dynamic)
                .insert(Velocity::default())
                .insert(GravityScale(1.0));
        }
    }

    // Cancel with Escape
    if game_input.cancel {
        for (entity, _, _) in held_query.iter() {
            commands
                .entity(entity)
                .remove::<HeldObject>()
                .insert(RigidBody::Dynamic)
                .insert(Velocity::default())
                .insert(GravityScale(1.0));
        }
    }
}
