use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

pub fn jumper_jumps (
    keyboard_input: Res<Input<KeyCode>>,
    mut jumpers: Query<(&mut Jumper, &mut RigidBodyVelocityComponent)>,
) {
    for (mut jumper, mut velocity) in jumpers.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0.0, jumper.jump_impulse).into();
            jumper.is_jumping = true;
        }
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

fn set_jumping_false_if_touching_floor(
    entity: Entity,
    jumper: &mut Jumper,
    event: &ContactEvent,
) {
    if let ContactEvent::Started(h1, h2) = event {
        if h1.entity() == entity || h2.entity() == entity {
            jumper.is_jumping = false;
        }
    }
}