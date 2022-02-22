use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("map_setup", SystemStage::single(spawn_floor));
    }
}

fn spawn_floor(mut commands: Commands) {
    let world = create_world(150);
    add_sprites(&mut commands, &world);
    add_colliders(&mut commands, &world);
}

fn create_world(width: usize) -> Vec<usize> {
    let mut heights: Vec<usize> = Vec::with_capacity(width);
    let mut height = 1;
    (0..width).for_each(|_| {
        heights.push(height);
        height = get_next_height(height);
    });
    heights
}

fn add_sprites(commands: &mut Commands, world: &Vec<usize>) {
    world.iter().enumerate().for_each(|(x, height)| {
        add_tile(commands, x as f32, *height as f32);
    })
}

fn add_tile(commands: &mut Commands, x: f32, height: f32) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(1.0, height as f32, 1.0),
                translation: Vec3::new(x, height as f32 / 2.0 -2.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn add_colliders(commands: &mut Commands, world: &Vec<usize>) {
    let max = match world.iter().max() {
        Some(m) => m,
        _ => panic!("add_colliders: World is empty!"),
    };
    (1..=*max).for_each(|floor_height| {
        let mut start: Option<usize> = None;
        world.iter().enumerate().for_each(|(index, height_at_index)| {
            if *height_at_index >= floor_height && start.is_none() {
                start = Some(index);
            } else if *height_at_index < floor_height && start.is_some() {
                add_collider(commands, floor_height, *start.get_or_insert(0), index);
                start = None;
            }
        });

        if start.is_some() {
            add_collider(commands, floor_height, *start.get_or_insert(0), world.len());
        }
    });
}

fn add_collider(commands: &mut Commands, height: usize, from: usize, to: usize) {
    let width = to - from;
    let half_width = width as f32 / 2.0;
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(from as f32 + half_width - 0.5, height as f32 - 2.5, 1.0).into(),
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(half_width, 0.5).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

fn get_random_height_delta() -> i8 {
    let mut rng = thread_rng();
    let random_number: u8 = rng.gen_range(0..100);
    let delta = match random_number {
        0..=70 => 0,
        71..=80 => -1,
        81..=90 => 1,
        _ => 2,
    };
    delta
}

fn get_next_height(current_height: usize) -> usize {
    let next_height = current_height as i8 + get_random_height_delta();
    return if next_height > 0 {
        next_height as usize
    } else {
        1
    };
}