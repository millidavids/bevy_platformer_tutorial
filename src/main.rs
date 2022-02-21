mod camera;
mod characters;
mod floor;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::*;
use characters::CharacterPlugin;
use floor::FloorPlugin;

fn main() {
    // initialize app object
    let mut app = App::new();

    // add plugins
    // TODO: separate out into bundles
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(FloorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));

    // run app
    app.run();
}