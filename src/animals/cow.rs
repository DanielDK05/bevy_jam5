use bevy::prelude::*;

use super::controller::AnimalMovementController;

pub(crate) struct CowPlugin;

impl Plugin for CowPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub(crate) struct CowMovementModifier {
    // I will never change this name
    pub(crate) udder_strength: f32,
}

#[derive(Bundle)]
pub(crate) struct CowBundle {
    pub(crate) movement_controller: AnimalMovementController,
    pub(crate) movement_modifier: CowMovementModifier,
    pub(crate) sprite_bundle: SpriteBundle,
}

mod systems {}
