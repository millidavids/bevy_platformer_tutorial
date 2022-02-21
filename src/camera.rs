use bevy::{
    app::App,
    prelude::{OrthographicCameraBundle, Plugin, Commands},
    render::camera::{
        OrthographicProjection,
        DepthCalculation,
        ScalingMode
    },
    math::Vec3
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(new_camera_2d());
}

fn new_camera_2d() -> OrthographicCameraBundle {
    let far = 1000.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedHorizontal,
        ..Default::default()
    };
    camera.transform.scale = Vec3::new(10.0, 10.0, 1.0);
    
    camera
}