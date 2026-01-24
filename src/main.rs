use bevy::{asset::AssetMetaCheck, prelude::*};
use stones::AppPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // https://github.com/bevyengine/bevy/issues/18002
                    // https://github.com/bevyengine/bevy/pull/10623
                    meta_check: AssetMetaCheck::Never,
                    //file_path: "assets".into(),
                    ..default()
                }),
            AppPlugin,
        ))
        .run();
}
