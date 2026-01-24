use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};

use crate::AppState;
use bevy::color::palettes::css::{BLACK, BLUE, WHITE};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menu)
            .add_systems(Update, update_menu.run_if(in_state(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((Camera2d, DespawnOnExit(AppState::Menu)));
    let container = (
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        DespawnOnExit(AppState::Menu),
    );

    let button = (
        Button,
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
            Text::new("Play"),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextFont::default().with_font_size(40.0),
        )],
    );

    let button_entity = commands.spawn(button).id();

    commands.spawn(container).add_children(&[button_entity]);
}

fn update_menu(
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut texts: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interactions {
        if let Ok(mut text) = texts.get_mut(children[0]) {
            match *interaction {
                Interaction::Pressed => {
                    text.0 = "Press".to_string();
                    *color = BLUE.into();
                    border_color.set_all(BLUE);
                    next_state.set(AppState::Playing);
                }
                Interaction::Hovered => {
                    text.0 = "Hover".to_string();
                    *color = RED.into();
                    border_color.set_all(WHITE);
                }
                Interaction::None => {
                    text.0 = "Button".to_string();
                    *color = GREEN.into();
                    border_color.set_all(BLACK);
                }
            }
        }
    }
}
