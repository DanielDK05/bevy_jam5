use bevy::prelude::*;

pub(crate) mod controller;
pub(crate) mod cow;

pub(crate) struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((controller::AnimalMovementControllerPlugin, cow::CowPlugin));
    }
}
