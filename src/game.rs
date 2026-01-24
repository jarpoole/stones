use avian3d::prelude::*;
use bevy::picking::hover::PickingInteraction;
use bevy::prelude::*;

use crate::AppState;

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
        children![
            // Static physics object with a collision shape
            (
                RigidBody::Static,
                Collider::cylinder(4.0, 0.1),
                Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Restitution::new(0.2),
            ),
            // GLTF
            (
                SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board.glb"))
                ),
                RigidBody::Dynamic,
                ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
                AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
                Transform::from_xyz(0.0, 6.0, 0.0).with_scale((0.1, 0.1, 0.1).into()),
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
