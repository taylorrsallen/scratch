//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct SelectPlugin;
impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectEvent>()
            .add_systems((
                    evsys_receive_select_events,
                ).in_base_set(PlayerSet::Actions));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    from: IVec3,
    to: IVec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaShape {
    Rect { start: IVec3, end: IVec3 },
    Sphere { origin: IVec3, radius: u32 },
    Cylinder { origin: IVec3, radius: u32, height: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionType {
    Coord  = 0x01,
    Coords = 0x02,
    Area   = 0x04,
    Areas  = 0x08,
    Unit   = 0x10,
    Units  = 0x20,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    Coord(CoordSelection),
    Coords(Vec<CoordSelection>),
    Area(AreaShape),
    Areas(Vec<AreaShape>),
    Unit(Entity),
    Units(Vec<Entity>),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
//================================-================================-================================
// CoordSelection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoordSelection {
    coord: IVec3,
    normal: IVec3,
}

impl CoordSelection {
    pub fn new(coord: &IVec3, normal: &IVec3) -> Self {
        Self { coord: *coord, normal: *normal }
    }

    pub fn coord(&self) -> &IVec3 { &self.coord }
    pub fn normal(&self) -> &IVec3 { &self.normal }
    pub fn coord_plus_normal(&self) -> IVec3 { self.coord + self.normal }
    pub fn coord_vec3(&self) -> Vec3 { self.coord.as_vec3() }
    pub fn normal_vec3(&self) -> Vec3 { self.normal.as_vec3() }
    pub fn coord_plus_scaled_normal(&self, scale: f32) -> Vec3 { self.coord.as_vec3() + self.normal.as_vec3() * scale }
}

//================================-================================-================================
// RaySelection
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RaySelection {
    coord_selection: CoordSelection,
    unit: Option<Entity>,
}

impl RaySelection {
    fn new(coord: &IVec3, normal: &IVec3, entity: &Option<Entity>) -> Self {
        Self { coord_selection: CoordSelection::new(coord, normal), unit: *entity }
    }

    pub fn coord_selection(&self) -> &CoordSelection { &self.coord_selection }
    pub fn unit(&self) -> &Option<Entity> { &self.unit }

    pub fn try_from_screenspace_raycast(
        player: &Player,
        window: &Window,
        entities: &mut Accessor<Option<Entity>>,
        camera_query: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
        rapier_context: &Res<RapierContext>,
    ) -> Option<(Self)> {
        if let Ok((camera, camera_global_transform)) = camera_query.get(player.camera) {
            if let Some(cursor_position) = window.cursor_position() {
                let ray = camera.viewport_to_world(camera_global_transform, cursor_position).unwrap();
                if let Some((entity, ray_intersection)) = rapier_context.cast_ray_and_get_normal(ray.origin, ray.direction, 200.0, true, QueryFilter::new()) {
                    let hit_point = ray.origin + ray.direction * ray_intersection.toi;

                    let coord = (hit_point + ray_intersection.normal * -0.1).round().as_ivec3();
                    let normal = ray_intersection.normal.as_ivec3();
                    let entity = entities.get_value(&coord);

                    return Some(Self::new(&coord, &normal, &entity));
                }
            }
        }

        None
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub enum SelectKind {
    Unit(Entity),
    Voxel(CoordSelection),
    StartSelection(CoordSelection),
    DragSelection(RaySelection),
}

pub struct SelectEvent {
    sender: Entity,
    kind: SelectKind,
}

impl SelectEvent {
    pub fn new(sender: Entity, kind: SelectKind) -> Self {
        Self { sender, kind }
    }

    pub fn unit(sender: Entity, select: Entity) -> Self {
        Self { sender, kind: SelectKind::Unit(select) }
    }

    pub fn voxel(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: SelectKind::Voxel(select) }
    }

    pub fn start_selection(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: SelectKind::StartSelection(select) }
    }

    pub fn drag_selection(sender: Entity, select: RaySelection) -> Self {
        Self { sender, kind: SelectKind::DragSelection(select) }
    }

    pub fn select(
        &self,
        selection: &mut PlayerSelector,
        spawn_events: &mut EventWriter<SpawnEvent>,
        marker_events: &mut EventWriter<MarkerEvent>,
        sound_2d_events: &mut EventWriter<Sound2dEvent>,
        voxel_events: &mut EventWriter<VoxelEvent>,
        selectable_query: &mut Query<&mut UnitSelectable>,
        input_state: &Res<InputState>,
    ) {
        match self.kind {
            SelectKind::Unit(entity) => {
                if let Ok(mut selectable) = selectable_query.get_mut(entity) { selectable.selected = true; }
                selection.select_unit(&entity, marker_events);
                spawn_events.send(SpawnEvent::UnitPropertiesDisplay(entity));
            }
            SelectKind::Voxel(select) => {
                if input_state.pressed(InputAction::SlowMod) {
                    voxel_events.send(VoxelEvent::Set(select));
                } else {
                    voxel_events.send(VoxelEvent::Create(select, selection.placing_voxel));
                }

                // selection.select_voxel(&select, marker_events);
            }
            SelectKind::StartSelection(select) => { /* println!("StartSelection"); */ }
            SelectKind::DragSelection(select) => {
                if select.unit.is_none() && input_state.pressed(InputAction::Z) {
                    voxel_events.send(VoxelEvent::Create(select.coord_selection, selection.placing_voxel));
                }
                /* println!("DragSelection"); */ 
            }
        }
    }

}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_receive_select_events(
    mut select_events: EventReader<SelectEvent>,
    mut voxel_events: EventWriter<VoxelEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut marker_events: EventWriter<MarkerEvent>,
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    mut selection_query: Query<&mut PlayerSelector>,
    mut selectable_query: Query<&mut UnitSelectable>,
    input_state: Res<InputState>,
) {
    for select_event in select_events.iter() {
        if let Ok(mut selection) = selection_query.get_mut(select_event.sender) {
            if !input_state.pressed(InputAction::MultiMod) { selection.deselect_all(&mut marker_events, &mut spawn_events, &mut selectable_query); }
            select_event.select(&mut selection, &mut spawn_events, &mut marker_events, &mut sound_2d_events, &mut voxel_events, &mut selectable_query, &input_state);
        }
    }
}