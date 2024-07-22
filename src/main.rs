use animals::{
    controller::AnimalMovementController,
    cow::{CowBundle, CowMovementModifier},
};
use avian2d::PhysicsPlugins;
use bevy::{color::palettes::css::BROWN, prelude::*};

mod animals;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(animals::AnimalsPlugin)
        .add_systems(Startup, test_setup)
        .run();
}

fn test_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(CowBundle {
        movement_controller: AnimalMovementController {
            speed: 1.0,
            jump_height: 1.0,
        },
        movement_modifier: CowMovementModifier {
            udder_strength: 1.0,
        },
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::from(BROWN),
                custom_size: Some(Vec2::ONE),
                ..default()
            },
            ..Default::default()
        },
    });
}
