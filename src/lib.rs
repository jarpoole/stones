use bevy::prelude::*;

mod camera;
mod controls;
mod game;
mod hud;
mod menu;
mod physics;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins((
            camera::CameraPlugin,
            controls::ControlsPlugin,
            physics::PhysicsPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
            hud::HudPlugin,
        ));
    }
}
