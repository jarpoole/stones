use bevy::prelude::*;

mod camera;
mod game;
mod hud;
mod menu;
mod physics;
mod picker;

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
            picker::PickerPlugin,
            physics::PhysicsPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
            hud::HudPlugin,
        ));
    }
}
