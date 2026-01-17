use bevy::prelude::*;

mod camera;
mod physics;
mod picker;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            picker::PickerPlugin,
            physics::PhysicsPlugin,
        ));
    }
}
