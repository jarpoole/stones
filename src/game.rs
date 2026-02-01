use avian3d::prelude::*;
use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct Gameboard;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        DespawnOnExit(AppState::Playing),
        // need to explicitly set visibility because the board collider must be explicitly hidden
        // https://bevy.org/learn/errors/b0004/
        Visibility::Visible,
        Name::new("Root"),
        children![
            (
                Gameboard,
                SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board.glb"))
                ),
                Name::new("Board"),
                Pickable {
                    should_block_lower: true,
                    ..default()
                },
                children![(
                    Name::new("Collider"),
                    RigidBody::Static,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("models/board_collider.glb")),
                    ),
                    Restitution::new(0.0),
                    ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0.0, 0.025, 0.0),
                    Visibility::Hidden,
                    //Observer::new(on_board_click_handler) //Pickable::IGNORE,
                )]
            ),
            // Light
            (
                PointLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(4.0, 8.0, 4.0),
            )
        ],
    ));
}

/*
fn on_board_click_handler(trigger: On<Pointer<Click>>, query: Query<&Name>) {
    if let Ok(name) = query.get(trigger.observer()) {
        println!("Clicked {}", name);
    }
}
    */
