mod characters;
mod map;
pub mod camera;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use characters::CharacterPlugin;
use map::MapPlugin;

fn main() {
    // initialize app object
    let mut app = App::new();

    // add plugins
    // TODO: separate out into bundles
    app.add_plugins(DefaultPlugins)
        .add_plugin(CharacterPlugin)
        .add_plugin(MapPlugin)
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
