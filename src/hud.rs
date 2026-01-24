use bevy::prelude::*;

use crate::AppState;

pub struct HudPlugin;

#[derive(Component)]
struct PauseButton;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup)
            .add_systems(Update, update_hud.run_if(in_state(AppState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let button = (
        Button,
        PauseButton,
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::WHITE),
        Node {
            width: px(150.0),
            height: px(65.0),
            border: UiRect::all(px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new("Pause"),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextFont::default().with_font_size(40.0),
        )],
    );

    commands.spawn(button).id();
}

fn update_hud(
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
    mut texts: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let interaction = interactions.single();
    if let Ok(Interaction::Pressed) = interaction {
        next_state.set(AppState::Menu);
        info!("hello");
    }
}
