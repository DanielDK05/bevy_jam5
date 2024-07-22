use bevy::prelude::*;

pub(super) struct AnimalMovementControllerPlugin;

impl Plugin for AnimalMovementControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::handle_animal_movement);
    }
}

#[derive(Component)]
pub(crate) struct AnimalMovementController {
    pub(crate) speed: f32,
    pub(crate) jump_height: f32,
}

mod systems {
    use avian2d::dynamics::rigid_body::LinearVelocity;
    use bevy::prelude::*;

    use super::AnimalMovementController;

    pub(super) fn handle_animal_movement(
        mut controller_query: Query<(&mut LinearVelocity, &AnimalMovementController)>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        for (mut linear_velocity, controller) in controller_query.iter_mut() {
            let direction = input
                .get_pressed()
                .map(|key| match key {
                    KeyCode::KeyA => Vec2::NEG_X,
                    KeyCode::KeyD => Vec2::X,
                    _ => Vec2::ZERO,
                })
                .sum::<Vec2>();

            **linear_velocity += direction.normalize_or_zero() * controller.speed;

            if input.just_pressed(KeyCode::Space) {
                linear_velocity.y = controller.jump_height;
            }
        }
    }
}
