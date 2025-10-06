use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::input::GameInput;
use crate::game::Tree;
use rand::Rng;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (detect_interactable, chop_tree_system));
    }
}

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
pub struct Highlighted;

#[derive(Component)]
pub struct Log;

fn detect_interactable(
    mut commands: Commands,
    _game_input: Res<GameInput>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    rapier_context: ReadRapierContext,
    interactable_query: Query<Entity, With<Interactable>>,
    highlighted_query: Query<Entity, With<Highlighted>>,
    windows: Query<&Window>,
) {
    // Remove old highlights
    for entity in highlighted_query.iter() {
        commands.entity(entity).remove::<Highlighted>();
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Convert cursor to world position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };

    let Ok(context) = rapier_context.single() else {
        return;
    };

    // Raycast in 2D - use the ray origin as a point on the 2D plane
    let point = ray.origin.truncate();
    let direction = Vec2::new(0.0, -1.0); // Cast downward
    let max_distance = 1000.0;

    if let Some((entity, _toi)) = context.cast_ray(point, direction, max_distance, true, QueryFilter::default()) {
        if interactable_query.contains(entity) {
            commands.entity(entity).insert(Highlighted);
        }
    }

    // Also try casting at the mouse cursor position directly
    if let Some((entity, _toi)) = context.cast_ray(point, Vec2::X, 50.0, true, QueryFilter::default()) {
        if interactable_query.contains(entity) {
            commands.entity(entity).insert(Highlighted);
        }
    }
}

fn chop_tree_system(
    mut commands: Commands,
    game_input: Res<GameInput>,
    mut tree_query: Query<(Entity, &mut Tree, &Transform, &Highlighted)>,
) {
    if !game_input.chop {
        return;
    }

    for (entity, mut tree, transform, _) in tree_query.iter_mut() {
        tree.health -= 34.0;

        if tree.health <= 0.0 {
            // Tree destroyed - spawn logs
            spawn_logs(&mut commands, transform.translation.truncate());
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_logs(commands: &mut Commands, position: Vec2) {
    let mut rng = rand::thread_rng();
    let num_logs = rng.gen_range(3..=6);

    for i in 0..num_logs {
        let length = rng.gen_range(30.0..=60.0);
        let width = 15.0;

        // Random offset
        let offset_x = rng.gen_range(-30.0..=30.0);
        let offset_y = i as f32 * 20.0 + 50.0;
        let spawn_pos = position + Vec2::new(offset_x, offset_y);

        // Random rotation
        let random_rotation = rng.gen_range(0.0..std::f32::consts::TAU);

        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.3, 0.2),
                custom_size: Some(Vec2::new(length, width)),
                ..default()
            },
            Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0)
                .with_rotation(Quat::from_rotation_z(random_rotation)),
            Log,
            Collider::cuboid(length / 2.0, width / 2.0),
            RigidBody::Dynamic,
            Interactable,
            Restitution::coefficient(0.3),
            Friction::coefficient(0.8),
            GravityScale(1.0),
        ));
    }
}
