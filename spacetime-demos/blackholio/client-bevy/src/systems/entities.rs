use bevy::prelude::*;
use crate::{
    components::*,
    utils::*,
    stdb::{Circle, Entity as DbEntity, Food},
};

pub fn spawn_circle(
    commands: &mut Commands,
    _circle: &Circle,
    entity: &DbEntity,
    _player_entity: Entity,
    player_id: u32,
    username: &str,
) -> Entity {
    let position: Vec2 = entity.position.clone().into();
    let color = CIRCLE_COLORS[(player_id as usize) % CIRCLE_COLORS.len()];

    let circle_entity = commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(mass_to_diameter(entity.mass))),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        EntityController::new(entity.entity_id, position),
        CircleController {
            player_id,
        },
        Name::new(format!("Circle-{}", entity.entity_id)),
    ))
    .with_children(|parent| {
        // Add text label for username
        parent.spawn((
            Text2d::new(username),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));
    })
    .id();

    circle_entity
}

pub fn spawn_food(
    commands: &mut Commands,
    _food: &Food,
    entity: &DbEntity,
) -> Entity {
    let position: Vec2 = entity.position.clone().into();
    let color = FOOD_COLORS[(entity.entity_id as usize) % FOOD_COLORS.len()];

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(mass_to_diameter(entity.mass))),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        EntityController::new(entity.entity_id, position),
        FoodController,
        Name::new(format!("Food-{}", entity.entity_id)),
    )).id()
}

pub fn update_entity_positions(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut EntityController)>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        // Interpolate position
        controller.lerp_time = (controller.lerp_time + time.delta_secs()).min(LERP_DURATION);
        let t = controller.lerp_time / LERP_DURATION;

        transform.translation = controller.lerp_start.lerp(controller.lerp_target, t);

        // Smoothly interpolate scale
        let current_scale = transform.scale;
        transform.scale = current_scale.lerp(controller.target_scale, time.delta_secs() * 8.0);
    }
}

pub fn handle_entity_update(
    entity_id: u32,
    new_position: Vec2,
    new_mass: u32,
    mut query: Query<(&mut EntityController, &mut Sprite)>,
) {
    for (mut controller, mut sprite) in query.iter_mut() {
        if controller.entity_id == entity_id {
            // Update lerp targets
            controller.lerp_time = 0.0;
            controller.lerp_start = Vec3::new(
                controller.lerp_target.x,
                controller.lerp_target.y,
                0.0,
            );
            controller.lerp_target = new_position.extend(0.0);
            controller.target_scale = mass_to_scale(new_mass);

            // Update sprite size
            sprite.custom_size = Some(Vec2::splat(mass_to_diameter(new_mass)));
            break;
        }
    }
}