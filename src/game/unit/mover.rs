//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use std::collections::VecDeque;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitMoverPlugin;
impl Plugin for UnitMoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
                    sys_update_mover_path,
                    sys_move_unit,
                ).chain().in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((
                    sys_draw_mover_path,
                ));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
pub enum UnitMoveType {
    None     = 0x00,
    /// Walk, Fall, & Jump
    Walk     = 0x01,
    /// Climb climbable walls & hang from climbable ledges & ceilings
    Climb    = 0x02,
    /// Walk on walls & ceilings
    Stick    = 0x04,
    /// Remain up to 1 voxel above last standing surface
    Levitate = 0x08,
    /// Walk through air while falling at cost of 1 height
    Glide    = 0x10,
    /// Move through any empty voxel
    Fly      = 0x20,
    /// Move instantly from coord to target, limited by speed as range
    Teleport = 0x40,
}

pub enum UnitMoveState {
    None     = 0x00,
    /// Unit is fully in control of movement
    Grounded = 0x01,
    /// Jumping or Falling, will attempt landing
    Inactive = 0x02,
    /// 
    Sliding  = 0x04,
    Tumbling = 0x08,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct UnitMove {
    move_type: UnitMoveType,
    target: IVec3,
}

impl UnitMove {
    pub fn new(move_type: UnitMoveType, target: &IVec3) -> Self {
        Self { move_type, target: *target }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
/// Unit can use `MoveActions`
/// 
/// Used by `UnitOrders` to assign `UnitMoveActions`
#[derive(Component)]
pub struct UnitMover {
    move_queue: VecDeque<UnitMove>,
    path: Vec<IVec3>,
    actions: u8,
    state: u8,
    max_jump: u8,
    max_fall: u8,
    speed: u8,
    move_acc: u32,
    move_delay: u8,
    fall_height: u16,
}

impl Default for UnitMover {
    fn default() -> Self {
        Self {
            move_queue: VecDeque::new(),
            path: vec![],
            actions: 0,
            state: 0,
            max_jump: 2,
            max_fall: 1,
            speed: 240,
            move_acc: 0,
            move_delay: 2,
            fall_height: 0,
        }
    }
}

impl UnitMover {
    pub fn new(actions: u8, max_jump: u8, max_fall: u8, speed: u8) -> Self {
        Self {
            move_queue: VecDeque::new(),
            path: vec![],
            actions: 0,
            state: 0,
            max_jump,
            max_fall,
            speed,
            move_acc: 0,
            move_delay: 2,
            fall_height: 0,
        }
    }

    pub fn max_jump(&self) -> u8 { self.max_jump }
    pub fn max_fall(&self) -> u8 { self.max_fall }
    pub fn steps_per_voxel(&self) -> u32 { self.move_delay as u32 } // (((255 - self.speed) as f32 / 255.0) * 30.0) as u32 }

    pub fn set_movement(&mut self, unit_move: UnitMove) {
        self.move_queue.clear();
        self.queue_movement(unit_move);
    }

    pub fn set_new_movement(&mut self, move_type: UnitMoveType, target: &IVec3) {
        self.move_queue.clear();
        self.queue_new_movement(move_type, target);
    }

    pub fn queue_movement(&mut self, unit_move: UnitMove) {
        self.move_queue.push_back(unit_move);
    }

    pub fn queue_new_movement(&mut self, move_type: UnitMoveType, target: &IVec3) {
        self.move_queue.push_back(UnitMove::new(move_type, target));
    }

    pub fn is_path_valid(
        &self,
        unit: &Unit,
        target: &IVec3,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        defs: &Res<Defs>,
    ) -> bool {
        if let Some(path_target) = self.path.first() {
            if *path_target != *target { return false; }
        } else { return false; }

        for coord in self.path.iter() {
            let voxel = voxels.get_value(coord);
            if !voxel.is_walkable(coord, voxels, defs) || !entities.is_empty(coord) { return false; }
        }

        true
    }

    pub fn path_len(&self) -> u32 {
        self.path.len() as u32
    }

    pub fn set_path(&mut self, path: &Vec<IVec3>) {
        self.path = path.clone();
        self.path.reverse();
        self.path.pop();
    }

    pub fn regenerate_path(
        &mut self,
        unit: &Unit,
        target: &IVec3,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        state_query: &Query<&UnitState>,
        defs: &Res<Defs>,
    ) -> bool {
        if let Some((path, cost)) = AStar::get_ground_path(unit, self.max_jump, self.max_fall, target, voxels, entities, state_query, defs) {
            self.set_path(&path);
            return true;
        } else {
            return false;
        }
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn steps_until_finished(
        &self,
    ) -> u32 {
        self.steps_per_voxel() * self.path_len()
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_mover_path(
    mut order_events: EventWriter<OrderEvent>,
    mut mover_query: Query<(Entity, &mut UnitMover, &mut UnitOrderable)>,
    unit_query: Query<&Unit>,
    state_query: Query<&UnitState>,
    level_tree_query: Query<(&VoxelTree, &EntityTree), With<LevelTree>>,
    game_time: Res<GameTime>,
    defs: Res<Defs>,
) {
    let (voxel_tree, entity_tree) = level_tree_query.single();
    mover_query.par_iter_mut().for_each_mut(|(this_entity, mut mover, mut orderable)| {
        let mut voxels = voxel_tree.get_accessor();
        let mut entities = entity_tree.get_accessor();
        let mut order_complete = false;
        let mut new_move_order = None;
        if let Some(move_order) = orderable.next_move_order() {
            let unit = if let Ok(unit) = unit_query.get(this_entity) { unit } else { return; };
            match move_order.target() {
                OrderTarget::Unit(target_entity) => {
                    if let Ok(target_unit) = unit_query.get(*target_entity) {
                        let neighbor_tiles = voxels.get_adjacent_side_values(target_unit.coord());
                        let mut closest = (None, u32::MAX);
                        for (i, tile) in neighbor_tiles.iter().enumerate() {
                            let tile_coord = *target_unit.coord() + VOXEL_SIDE_FACE_CHECKS[i];
                            let distance = AStarNode::distance(unit.coord(), &tile_coord);
                            if distance < closest.1 && tile.is_walkable(&tile_coord, &mut voxels, &defs) && entities.is_empty(&tile_coord) {
                                closest = (Some(tile_coord), distance);
                            }
                        }

                        if let Some(tile_coord) = closest.0 {
                            println!("Completed [Move: Unit] Order");
                            new_move_order = Some(MoveOrder::new(OrderTarget::Voxel(CoordSelection::new(&tile_coord, &IVec3::ZERO))));
                        }
                    } else {
                        println!("Abandoned [Move: Unit] Order");
                        order_complete = true;
                    }
                }
                OrderTarget::Voxel(select) => {
                    if *unit.coord() == select.coord_plus_normal() {
                        println!("Completed [Move: Voxel] Order");
                        order_complete = true;
                    } else if !mover.is_path_valid(unit, &select.coord_plus_normal(), &mut voxels, &mut entities, &defs) {
                        if !mover.regenerate_path(unit, &select.coord_plus_normal(), &mut voxels, &mut entities, &state_query, &defs) {
                            println!("Abandoned [Move: Voxel] Order");
                            order_complete = true;
                        }
                    }
                }
            }
        } else {
            return;
        }

        if order_complete {
            orderable.pop_move_order();
        }

        if let Some(new_move_order) = new_move_order {
            orderable.clear_move_orders();
            orderable.queue_move_order(&new_move_order);
        }
    });
}

fn sys_move_unit(
    mut mover_query: Query<(Entity, &mut UnitMover, &mut Unit)>,
    mut health_events: EventWriter<HealthEvent>,
    level_tree_query: Query<(&VoxelTree, &EntityTree), With<LevelTree>>,
    game_time: Res<GameTime>,
    defs: Res<Defs>,
) {
    let (voxel_tree, entity_tree) = level_tree_query.single();
    let mut voxels = voxel_tree.get_accessor();
    let mut entities = entity_tree.get_accessor();
    for (entity, mut mover, mut unit) in mover_query.iter_mut() {
        if !voxels.adjacent_value_from_direction(unit.coord(), GridDirection::Bottom).is_solid(&defs) {
            if entities.adjacent_value_from_direction(unit.coord(), GridDirection::Bottom).is_some() {
                mover.clear_path();
            } else {
                mover.fall_height += 1;
                entities.set_value_off(unit.coord());
                entities.set_value_on(&(*unit.coord() - IVec3::Y), &Some(entity));
                unit.coord -= IVec3::Y;
            }

            continue;
        } else if mover.fall_height > 0 {
            health_events.send(HealthEvent::Sub(entity, mover.fall_height as u32));
            mover.fall_height = 0;
        }
        mover.move_acc += game_time.delta_steps();
        if mover.move_acc >= mover.steps_per_voxel() {
            mover.move_acc -= mover.steps_per_voxel();
            if let Some(coord) = mover.path.pop() {
                if entities.get_value(&coord).is_some() {
                    mover.clear_path();
                } else {
                    entities.set_value_off(unit.coord());
                    entities.set_value_on(&coord, &Some(entity));
                    unit.coord = coord;
                }
            }
        }
    }
}

fn sys_draw_mover_path(mover_query: Query<(&UnitMover, &Unit)>) {
    mover_query.par_iter().for_each(|(mover, unit)| {
        if mover.path.is_empty() { return; }
        draw_line(vec![unit.coord.as_vec3(), mover.path.last().unwrap().as_vec3()], Color::GREEN);
        draw_line(mover.path.iter().map(|vec| vec.as_vec3()).collect(), Color::GREEN);
    });
}