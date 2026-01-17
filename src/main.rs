use avian3d::prelude::*;
use bevy::{asset::AssetMetaCheck, picking::hover::PickingInteraction, prelude::*};
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
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Restitution::new(0.2),
    ));

    /*
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Fox.glb")),
        // https://github.com/bevyengine/bevy/discussions/17151
        //asset_server.load("models/Fox.glb"),
    ));
    */
    //commands.spawn(SceneRoot(asset_server.load("models/Fox.glb#Scene0")));

    // Dynamic physics object with a collision shape and initial angular velocity
    /*
    commands.spawn((
        RigidBody::Dynamic,
        //Collider::cuboid(1.0, 1.0, 1.0),
        ColliderConstructor::TrimeshFromMesh,
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 6.0, 0.0),
    ));
    */

    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/test.glb"))),
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Transform::from_xyz(0.0, 6.0, 0.0).with_scale((0.1, 0.1, 0.1).into()),
        PickingInteraction::default(),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
