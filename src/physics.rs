use avian3d::{PhysicsPlugins, prelude::PhysicsPickingPlugin};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            // https://docs.rs/avian3d/0.4.1/avian3d/picking/index.html
            PhysicsPickingPlugin,
        ));
    }
}
