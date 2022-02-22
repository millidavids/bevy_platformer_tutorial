use super::jumper::*;
use crate::camera::new_camera_2d;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

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
        .insert(Player { speed: 3.5 })
        .insert(Jumper {
            jump_impulse: 7.0,
            is_jumping: false,
        })
        .with_children(|parent| {
            parent.spawn_bundle(new_camera_2d());
        });
}

pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut RigidBodyVelocityComponent), With<Player>>
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) && !jumper.is_jumping{
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
            jumper.is_jumping = true;
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut RigidBodyVelocityComponent)>
) {
    for (player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
        }
    }
}