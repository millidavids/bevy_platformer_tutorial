use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage("floor_setup", SystemStage::single(spawn_floor));
    }
}

fn spawn_floor(mut commands: Commands) {
    let width = 10.0;
    let height = 1.0;
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, -2.0, 1.0).into(),
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(width / 2.0, height / 2.0).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(width, height, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}