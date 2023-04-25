//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct SelectorPlugin;
impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VoxelEvent>()
            .add_systems((
                    sys_update_selection,
                ).in_base_set(PlayerSet::Commands))
            .add_system(sys_voxel_events.in_base_set(PlayerSet::Last));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct PlayerSelector {
    cursor: Entity,
    selected_units: Vec<Entity>,
    selected_voxels: Vec<CoordSelection>,
    select_sound: Option<Sound2dEvent>,
    pub placing_voxel: Voxel,
}

impl PlayerSelector {
    pub fn new(
        cursor: Entity,
    ) -> Self {
        Self {
            cursor,
            selected_voxels: vec![],
            selected_units: vec![],
            select_sound: None,
            placing_voxel: Voxel::from_matter_id(1),
        }
    }

    pub fn cursor(&self) -> &Entity { &self.cursor }
    pub fn selected_units(&self) -> &Vec<Entity> { &self.selected_units }
    pub fn selected_voxels(&self) -> &Vec<CoordSelection> { &self.selected_voxels }

    pub fn select_unit(&mut self, unit: &Entity, marker_events: &mut EventWriter<MarkerEvent>) {
        println!("Select Unit");
        marker_events.send(MarkerEvent::Show(*unit));
        self.selected_units.push(*unit);
    }

    pub fn deselect_unit(&mut self, unit: Entity, marker_events: &mut EventWriter<MarkerEvent>) {
        let mut index = None;
        for (i, entity) in self.selected_units.iter().enumerate() {
            if *entity == unit {
                index = Some(i);
                marker_events.send(MarkerEvent::Hide(unit));
                break;
            }
        }

        if let Some(index) = index {
            self.selected_units.swap_remove(index);
        }
    }

    pub fn deselect_all(
        &mut self,
        marker_events: &mut EventWriter<MarkerEvent>,
        spawn_events: &mut EventWriter<SpawnEvent>,
        selectable_query: &mut Query<&mut UnitSelectable>,
    ) {
        for entity in self.selected_units.iter() {
            if let Ok(mut selectable) = selectable_query.get_mut(*entity) { selectable.selected = false; }
            marker_events.send(MarkerEvent::Hide(*entity));
        }
        self.selected_voxels.clear();
        self.selected_units.clear();
    }

    pub fn select_voxel(&mut self, voxel: &CoordSelection, marker_events: &mut EventWriter<MarkerEvent>) {
        println!("Select Voxel");
        // marker_events.send(MarkerEvent::Show(unit));
        self.selected_voxels.push(*voxel);
    }

    pub fn get_all_units_faction(&self, ) -> u16 {
        0
    }

    pub fn play_select_sound(&self, sound_2d_events: &mut EventWriter<Sound2dEvent>) {
        if let Some(sound_2d_event) = &self.select_sound { sound_2d_events.send(sound_2d_event.clone()); }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_selection(
    mut cursor_events: EventWriter<CursorEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut voxel_events: EventWriter<VoxelEvent>,
    mut designate_events: EventWriter<DesignateEvent>,
    mut select_events: EventWriter<SelectEvent>,
    mut order_events: EventWriter<OrderEvent>,
    player_query: Query<(Entity, &Player, &PlayerSelector)>,
    designator_query: Query<&PlayerDesignator>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    entity_tree_query: Query<&EntityTree, With<LevelTree>>,
    input_state: Res<InputState>,
    rapier_context: Res<RapierContext>,
) {
    let window = window_query.single();
    let mut entities = entity_tree_query.single().get_accessor();

    for (player_entity, player, selection) in player_query.iter() {
        let ray = if let Some(ray) = RaySelection::try_from_screenspace_raycast(&player, &window, &mut entities, &camera_query, &rapier_context) {
                cursor_events.send(CursorEvent::Move(selection.cursor, ray));
                ray
            } else {
                cursor_events.send(CursorEvent::Hide(selection.cursor));
                continue;
            };

        if let Ok(designator) = designator_query.get(player_entity) {
            if designator.is_designating() {
                send_designate_events(player_entity, &ray, &selection, &mut designate_events, &input_state);
                continue;
            }
        }

        if input_state.just_released(InputAction::Inventory) {
            spawn_events.send(SpawnEvent::Actor(ray.coord_selection().coord_plus_normal(), 1));
        } else if input_state.pressed(InputAction::Inventory) && input_state.pressed(InputAction::Z){

        }

        send_select_and_order_events(player_entity, &ray, &selection, &mut voxel_events, &mut select_events, &mut order_events, &input_state);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
fn send_designate_events(
    player_entity: Entity,
    ray: &RaySelection,
    selection: &PlayerSelector,
    designate_events: &mut EventWriter<DesignateEvent>,
    input_state: &Res<InputState>,
) {
    if input_state.just_released(InputAction::PrimaryAction) {
        if let Some(unit) = *ray.unit() {
            designate_events.send(DesignateEvent::unit(player_entity, unit));
        } else {
            designate_events.send(DesignateEvent::voxel(player_entity, *ray.coord_selection()));
        }
    } else if input_state.just_pressed(InputAction::PrimaryAction) {
        designate_events.send(DesignateEvent::start_selection(player_entity, *ray.coord_selection()));
    } else if input_state.pressed(InputAction::PrimaryAction) {
        designate_events.send(DesignateEvent::drag_selection(player_entity, *ray.coord_selection()));
    }

    if input_state.just_released(InputAction::SecondaryAction) {
        designate_events.send(DesignateEvent::cancel(player_entity));
    }
}

fn send_select_and_order_events(
    player_entity: Entity,
    ray: &RaySelection,
    selection: &PlayerSelector,
    voxel_events: &mut EventWriter<VoxelEvent>,
    select_events: &mut EventWriter<SelectEvent>,
    order_events: &mut EventWriter<OrderEvent>,
    input_state: &Res<InputState>,
) {
    if input_state.just_released(InputAction::PrimaryAction) {
        if let Some(unit) = *ray.unit() {
            select_events.send(SelectEvent::unit(player_entity, unit));
        } else {
            select_events.send(SelectEvent::voxel(player_entity, *ray.coord_selection()));
        }
    } else if input_state.just_pressed(InputAction::PrimaryAction) {
        select_events.send(SelectEvent::start_selection(player_entity, *ray.coord_selection()));
    } else if input_state.pressed(InputAction::PrimaryAction) {
        select_events.send(SelectEvent::drag_selection(player_entity, *ray));
    }

    if selection.selected_units.is_empty() {
        if input_state.just_released(InputAction::SecondaryAction) {
            voxel_events.send(VoxelEvent::Destroy(*ray.coord_selection()));
        } else if input_state.pressed(InputAction::SecondaryAction) && input_state.pressed(InputAction::Z) {
            voxel_events.send(VoxelEvent::Destroy(*ray.coord_selection()));
        }
        return;
    }

    if input_state.just_released(InputAction::SecondaryAction) {
        if let Some(unit) = *ray.unit() {
            order_events.send(OrderEvent::unit(player_entity, unit));
        } else {
            order_events.send(OrderEvent::voxel(player_entity, *ray.coord_selection()));
        }
    } else if input_state.just_pressed(InputAction::SecondaryAction) {
        order_events.send(OrderEvent::start_path(player_entity, *ray.coord_selection()));
    } else if input_state.pressed(InputAction::SecondaryAction) {
        order_events.send(OrderEvent::drag_path(player_entity, *ray.coord_selection()));
    }
}



/// Just hacking this in last minute because it was the best part of this unfinished thing
pub enum VoxelEvent {
    Destroy(CoordSelection),
    Create(CoordSelection, Voxel),
    Set(CoordSelection),
}

fn sys_voxel_events(
    mut voxel_events: EventReader<VoxelEvent>,
    mut selector_query: Query<&mut PlayerSelector>,
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    level_tree_query: Query<&VoxelTree, With<LevelTree>>,
    asset_loader: Res<AssetLoader>,
) {
    let mut voxels = level_tree_query.single().get_accessor();
    for voxel_event in voxel_events.iter() {
        match voxel_event {
            VoxelEvent::Destroy(coord_selection) => { voxels.set_voxel_off(coord_selection.coord()); }
            VoxelEvent::Create(coord_selection, voxel)  => { voxels.set_value_on(&coord_selection.coord_plus_normal(), voxel); }
            VoxelEvent::Set(coord_selection) => {
                if let Ok(mut selector) = selector_query.get_single_mut() {
                    selector.placing_voxel = voxels.get_value(coord_selection.coord());
                    sound_2d_events.send(Sound2dEvent::new(asset_loader.sounds.get_handle("8bit/magic_00"), 0.5));
                }
            }
        }
    }
}