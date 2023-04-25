//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitSelectablePlugin;
impl Plugin for UnitSelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sys_despawn_selection_visuals);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
/// Anything with this component will be despawned if the selectable is no longer selected
#[derive(Component)]
pub struct SelectedVisual(pub Entity);
/// Anything with this component will be despawned if the selectable is no longer hovered
#[derive(Component)]
pub struct HoveredVisual(pub Entity, pub bool);

/// This just means you can visually select a unit, e.g. they aren't terrain
#[derive(Component)]
pub struct UnitSelectable {
    pub selected: bool,
    pub marker: Entity,
}

impl UnitSelectable {
    pub fn new(marker: Entity) -> Self {
        Self { selected: false, marker }
    }
}

fn sys_despawn_selection_visuals(mut commands: Commands, visuals_query: Query<(Entity, &SelectedVisual)>, selectables_query: Query<&UnitSelectable>) {
    for (entity, visual) in visuals_query.iter() {
        if let Ok(selectable) = selectables_query.get(visual.0) {
            if !selectable.selected { commands.entity(entity).despawn_recursive(); }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// fn sys_despawn_hover_visuals(mut commands: Commands, visuals_query: Query<(Entity, &HoveredVisual)>, selectables_query: Query<&UnitSelectable>) {
//     for (entity, visual) in visuals_query.iter() {
//         if let Ok(selectable) = selectables_query.get(visual.0) {
//             if !selectable.hovered { commands.entity(entity).despawn_recursive(); }
//         } else {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }