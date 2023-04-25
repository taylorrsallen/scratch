//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct MarkerPlugin;
impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CursorEvent>()
            .add_event::<MarkerEvent>()
            .add_systems((
                    evsys_receive_cursor_events,
                    evsys_receive_marker_events,
                ).in_base_set(PlayerSet::Last));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct SelectionMarker;

impl SelectionMarker {
    pub fn hide(marker: &Entity, marker_query: &mut Query<&mut Visibility, With<Self>>) {
        if let Ok(mut marker_visibility) = marker_query.get_mut(*marker) { *marker_visibility = Visibility::Hidden; }
    }

    pub fn show(marker: &Entity, marker_query: &mut Query<&mut Visibility, With<Self>>) {
        if let Ok(mut marker_visibility) = marker_query.get_mut(*marker) { *marker_visibility = Visibility::Visible; }
    }

    pub fn try_get_selectable_marker(unit: &Entity, selectable_query: &Query<&UnitSelectable>) -> Option<Entity> {
        if let Ok(selectable) = selectable_query.get(*unit) {
            Some(selectable.marker)
        } else {
            None
        }
    }

    pub fn move_to_ray(
        marker: &Entity,
        ray: &RaySelection,
        transform_query: &mut Query<&mut Transform,With<SelectionMarker>>,
        unit_query: &Query<&Unit>,
    ) {
        let mut transform = if let Ok(transform) = transform_query.get_mut(*marker) { transform } else { return; };
        if let Some(entity) = *ray.unit() {
            SelectionMarker::move_to_unit(unit_query.get(entity).unwrap(), ray.coord_selection(), &mut transform);
        } else {
            SelectionMarker::move_to_voxel(ray.coord_selection(), &mut transform);
        }
    }
    
    pub fn move_to_unit(unit: &Unit, selection: &CoordSelection, transform: &mut Transform) {
        // We'll need the unit for when there are units with different scales
        transform.translation = selection.coord_vec3() - Vec3::Y * 0.5;
        transform.look_to(Vec3::NEG_Y, Vec3::X);
    }

    pub fn move_to_voxel(selection: &CoordSelection, transform: &mut Transform) {
        transform.translation = selection.coord_plus_scaled_normal(0.5);
        transform.look_to(-selection.normal_vec3(), selection.normal_vec3().any_orthogonal_vector());
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
/// Entity is the cursor
pub enum CursorEvent {
    Hide(Entity),
    Move(Entity, RaySelection),
}

/// Entity is the unit, marker is its child
pub enum MarkerEvent {
    Hide(Entity),
    Show(Entity),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_receive_cursor_events(
    mut cursor_events: EventReader<CursorEvent>,
    mut visible_query: Query<&mut Visibility, With<SelectionMarker>>,
    mut transform_query: Query<&mut Transform, With<SelectionMarker>>,
    unit_query: Query<&Unit>,
) {
    for cursor_event in cursor_events.iter() {
        match cursor_event {
            CursorEvent::Hide(cursor) => {
                SelectionMarker::hide(cursor, &mut visible_query);
            }
            CursorEvent::Move(cursor, ray) => {
                SelectionMarker::show(cursor, &mut visible_query);
                SelectionMarker::move_to_ray(cursor, ray, &mut transform_query, &unit_query);
            }
        }
    }
}

fn evsys_receive_marker_events(
    mut marker_events: EventReader<MarkerEvent>,
    mut visible_query: Query<&mut Visibility, With<SelectionMarker>>,
    selectable_query: Query<&UnitSelectable>,
    unit_query: Query<&Unit>,
) {
    for marker_event in marker_events.iter() {
        match marker_event {
            MarkerEvent::Hide(unit) => {
                let marker = if let Some(marker) = SelectionMarker::try_get_selectable_marker(&unit, &selectable_query) { marker } else { continue; };
                SelectionMarker::hide(&marker, &mut visible_query);
            }
            MarkerEvent::Show(unit) => {
                let marker = if let Some(marker) = SelectionMarker::try_get_selectable_marker(&unit, &selectable_query) { marker } else { continue; };
                SelectionMarker::show(&marker, &mut visible_query);
            }
        }
    }
}