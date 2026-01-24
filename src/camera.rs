use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(OnEnter(AppState::Playing), setup);
    }
}

fn setup(mut commands: Commands) {
    // https://docs.rs/bevy_panorbit_camera/latest/bevy_panorbit_camera/index.html
    commands.spawn((
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
        PanOrbitCamera {
            pitch_lower_limit: Some(0.0),
            allow_upside_down: false,
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));
}
