use avian3d::prelude::*;
use bevy::picking::hover::PickingInteraction;
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
    mut meshes: ResMut<Assets<Mesh>>,
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
        children![
            (
                Gameboard,
                SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board.glb"))
                ),
                children![(
                    RigidBody::Static,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("models/board_collider.glb")),
                    ),
                    Restitution::new(0.2),
                    ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0.0, 0.025, 0.0),
                    Visibility::Hidden
                )]
            ),
            // GLTF
            (
                SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/piece.glb"))
                ),
                RigidBody::Dynamic,
                ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh), // dramatically improves performance
                AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
                Transform::from_xyz(0.0, 6.0, 0.0).with_scale((1.3, 1.3, 1.3).into()),
                PickingInteraction::default(),
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
