use bevy::prelude::*;

pub struct PickerPlugin;

impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_color_on_click)
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
