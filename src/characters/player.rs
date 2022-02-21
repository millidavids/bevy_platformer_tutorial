use super::jumper::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

pub fn spawn_player(mut commands: Commands) {
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        activation: RigidBodyActivation::cannot_sleep().into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player)
        .insert(Jumper {
            jump_impulse: 10.0,
            is_jumping: false,
        });
}
