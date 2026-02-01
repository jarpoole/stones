use avian3d::PhysicsPlugins;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(),));
    }
}
