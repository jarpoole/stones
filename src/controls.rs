use avian3d::prelude::{
    AngularVelocity, Collider, ColliderConstructor, ColliderConstructorHierarchy, Restitution,
    RigidBody,
};
use bevy::{
    asset::AsAssetId,
    camera::primitives::{Aabb, MeshAabb},
    mesh::PrimitiveTopology,
    picking::hover::{self, PickingInteraction},
    prelude::*,
};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::game::{Gameboard, PieceVariant, SpawnPieceOptions, spawn_piece};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_color_on_click)
            .add_systems(Update, draw_cursor)
            .insert_resource(SelectedEntity(None))
            .add_plugins(MeshPickingPlugin);
    }
}

#[derive(Resource)]
struct SelectedEntity(Option<Entity>);

fn change_color_on_click(
    mut events: MessageReader<Pointer<Click>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut selected: ResMut<SelectedEntity>,
) {
    for event in events.read() {
        info!("hello");
        // reset currently selected entity
        if let Some(currently_selected_entity) = selected.0
            && let Ok(handle) = query.get(currently_selected_entity)
            && let Some(material) = materials.get_mut(handle)
        {
            material.base_color = Color::WHITE;
        }
        // set new color
        if selected.0 != Some(event.entity) {
            selected.0 = Some(event.entity);
            if let Ok(handle) = query.get(event.entity)
                && let Some(material) = materials.get_mut(handle)
            {
                material.base_color = Color::BLACK;
            }
        }
    }
}

/*
#[derive(Bundle)]
struct PieceBundle {
    name: Name,
    scene: SceneRoot,
    mode: RigidBody,
    collider: Collider,
    restitution: Restitution,
    transform: Transform,
}
impl Default for PieceBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Piece"),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/piece.glb")),
            ),
            mode: RigidBody::Dynamic,
            collider: ColliderConstructorHierarchy::new(
                // dramatically improves performance
                ColliderConstructor::ConvexDecompositionFromMesh,
            ),
            restitution: Restitution::new(0.0),
            transform: Transform::from_xyz(point.x, point.y, point.z)
                .with_scale((1.3, 1.3, 1.3).into()),
        }
    }
}
    */

fn draw_cursor(
    mut hover_events: MessageReader<Pointer<Move>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<(Entity, &GlobalTransform), With<Gameboard>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<(&Aabb, &GlobalTransform)>,
    //
    names: Query<&Name>,
    parents: Query<&ChildOf>,
    world: &World,
) {
    let (camera, camera_transform) = *camera_query;
    let (ground, ground_transform) = *ground;
    for hover_event in hover_events.read() {
        let mouse_position = hover_event.pointer_location.position;
        let point = hover_event.hit.position.expect("no position available");
        info!("hovered: {:?}", names.get(hover_event.entity));
        let x = world
            .inspect_entity(hover_event.entity)
            .into_iter()
            .map(|x| x.map(|y| y.name()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        info!("{:#?}", x);

        gizmos.circle(
            Isometry3d::new(
                point + ground_transform.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground_transform.up().as_vec3()),
            ),
            0.05,
            Color::WHITE,
        );
        /*
        if let Ok(ray) = camera.viewport_to_world(camera_transform, mouse_position)
            && let Some(distance) = ray.intersect_plane(
                ground_transform.translation(),
                InfinitePlane3d::new(ground_transform.up()),
            )
        {}

        let (entity_mesh, transform) = mesh_query.get(hover_event.entity).expect("");
        f let Some(mesh) = meshes.get(entity_mesh.id()) {
            if let Some(aabb) = mesh.compute_aabb() {
                info!("AABB: {:?}", aabb);
                //let world_aabb = aabb.transform(transform);
                //info!("World AABB: {:?}", world_aabb);
            }
        }
        */
        info!("AABB: {:?}", mesh_query.get(hover_event.entity));
    }
    /*
    let (camera, camera_transform) = *camera_query;
    let (ground, ground_transform) = *ground;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
        // Calculate if and at what distance the ray is hitting the ground plane.
        && let Some(distance) =
            ray.intersect_plane(ground_transform.translation(), InfinitePlane3d::new(ground_transform.up()))
    {
        let point = ray.get_point(distance);
        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + ground_transform.up() * 0.05,
                Quat::from_rotation_arc(Vec3::Z, ground_transform.up().as_vec3()),
            ),
            0.05,
            Color::WHITE,
        );

        for mouse_event in mouse_events.read() {
            info!("clicked : {:?}", names.get(mouse_event.entity));
            let mouse_position = mouse_event.pointer_location.position;
            if let Ok(ray) = camera.viewport_to_world(camera_transform, mouse_position)
                && let Some(distance) = ray.intersect_plane(
                    ground_transform.translation(),
                    InfinitePlane3d::new(ground_transform.up()),
                )
            {
                let point = ray.get_point(distance) + ground_transform.up() * 0.5;
            }
        }
    }
    */
}

fn on_click(
    mut mouse_events: MessageReader<Pointer<Click>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<(Entity, &GlobalTransform), With<Gameboard>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    names: Query<&Name>,
    parents: Query<&ChildOf>,
) {
    let (camera, camera_transform) = *camera_query;
    let (ground, ground_transform) = *ground;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
        // Calculate if and at what distance the ray is hitting the ground plane.
        && let Some(distance) =
            ray.intersect_plane(ground_transform.translation(), InfinitePlane3d::new(ground_transform.up()))
    {
        let point = ray.get_point(distance);

        for mouse_event in mouse_events.read() {
            info!("clicked : {:?}", names.get(mouse_event.entity));
            let mouse_position = mouse_event.pointer_location.position;
            if let Ok(ray) = camera.viewport_to_world(camera_transform, mouse_position)
                && let Some(distance) = ray.intersect_plane(
                    ground_transform.translation(),
                    InfinitePlane3d::new(ground_transform.up()),
                )
            {
                let point = ray.get_point(distance) + ground_transform.up() * 0.5;
                spawn_piece(
                    &mut commands,
                    &asset_server,
                    PieceVariant::NEUTRAL,
                    point.x,
                    point.y,
                    point.z,
                    1,
                );
                /*
                    commands.spawn((
                        Name::new("Piece"),
                        SceneRoot(
                            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/piece.glb")),
                        ),
                        RigidBody::Dynamic,
                        ColliderConstructorHierarchy::new(
                            // dramatically improves performance
                            ColliderConstructor::ConvexDecompositionFromMesh,
                        ),
                        Restitution::new(0.0),
                        Transform::from_xyz(point.x, point.y, point.z)
                            .with_scale((1.3, 1.3, 1.3).into()),
                    ));
                */
            }
        }
    }
}
