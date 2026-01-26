use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::game::Gameboard;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_color_on_click)
            .add_systems(Update, draw_cursor)
            .insert_resource(SelectedEntity(None));
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

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Gameboard>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
        // Calculate if and at what distance the ray is hitting the ground plane.
        && let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        let point = ray.get_point(distance);
        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            Color::WHITE,
        );
    }
}
