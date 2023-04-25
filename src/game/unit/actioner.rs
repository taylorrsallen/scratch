//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use std::collections::VecDeque;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitActionerPlugin;
impl Plugin for UnitActionerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
                sys_queue_actions,
                sys_update_actions,
            ).chain().in_schedule(CoreSchedule::FixedUpdate));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitActionKind {
    Attack(u8),
    Use(u8),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct UnitAction {
    kind: UnitActionKind,
    target: OrderTarget,
    cast: u32,
    acc: u32,
}

impl UnitAction {
    pub fn new(kind: UnitActionKind, target: &OrderTarget) -> Self {
        Self { kind, target: *target, cast: 10, acc: 0 }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct UnitActioner {
    action_queue: VecDeque<UnitAction>,
}

impl Default for UnitActioner {
    fn default() -> Self {
        Self { action_queue: VecDeque::new() }
    }
}

impl UnitActioner {
    pub fn new() -> Self {
        Self { action_queue: VecDeque::new() }
    }

    pub fn set_action(&mut self, unit_action: UnitAction) {
        self.action_queue.clear();
        self.queue_action(unit_action);
    }

    pub fn queue_action(&mut self, unit_action: UnitAction) {
        self.action_queue.push_back(unit_action);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_queue_actions(
    mut order_events: EventWriter<OrderEvent>,
    mut actioner_query: Query<(Entity, &mut UnitActioner, &mut UnitOrderable)>,
    unit_query: Query<&Unit>,
    state_query: Query<&UnitState>,
    level_tree_query: Query<(&VoxelTree, &EntityTree), With<LevelTree>>,
    game_time: Res<GameTime>,
    defs: Res<Defs>,
) {
    let (voxel_tree, entity_tree) = level_tree_query.single();
    actioner_query.par_iter_mut().for_each_mut(|(this_entity, mut actioner, mut orderable)| {
        let mut voxels = voxel_tree.get_accessor();
        let mut entities = entity_tree.get_accessor();
        let mut order_complete = false;
        let mut move_order = None;
        if let Some(action_order) = orderable.next_action_order() {
            match action_order.target() {
                OrderTarget::Unit(target_entity) => {
                    match action_order.kind() {
                        ActionOrderKind::Use => {
                            // if we're not in use range && no move orders, assign order
                            println!("[Use: Unit] Order");
                        }
                        ActionOrderKind::Attack => {
                            // if we're not in attack range && no move orders, assign order
                            if let Ok(target_unit) = unit_query.get(*target_entity) {
                                if let Ok(this_unit) = unit_query.get(this_entity) {
                                    let distance = AStarNode::distance(this_unit.coord(), target_unit.coord());
                                    if distance != 1 && orderable.next_move_order().is_none() {
                                        move_order = Some(MoveOrder::new(OrderTarget::Unit(*target_entity)));
                                    } else if distance == 1 {
                                        println!("Completed [Attack: Unit] Order");
                                        actioner.queue_action(UnitAction::new(UnitActionKind::Attack(5), action_order.target()));
                                        order_complete = true;
                                    }
                                }
                            } else {
                                println!("Abandoned [Attack: Unit] Order");
                                order_complete = true;
                            }
                        }
                        ActionOrderKind::Ability(id) => {
                            // if we're not in ability range && no move orders, assign order
                            println!("[Ability: Unit] Order");
                        }
                    }
                }
                OrderTarget::Voxel(select) => {
                    
                }
            }
        } else {
            return;
        }

        if let Some(move_order) = move_order {
            orderable.queue_move_order(&move_order);
        }

        if order_complete {
            orderable.pop_action_order();
        }
    });
}

fn sys_update_actions(
    mut health_events: EventWriter<HealthEvent>,
    mut sound_3d_events: EventWriter<Sound3dEvent>,
    mut actioner_query: Query<(Entity, &mut UnitActioner)>,
    unit_query: Query<(&Unit)>,
    level_tree_query: Query<&EntityTree, With<LevelTree>>,
    game_time: Res<GameTime>,
    asset_loader: Res<AssetLoader>,
) {
    let mut entities = level_tree_query.single().get_accessor();
    for (entity, mut actioner) in actioner_query.iter_mut() {
        let mut action_complete = false;
        if let Some(mut action) = actioner.action_queue.front_mut() {
            action.acc += game_time.delta_steps();
            if action.acc >= action.cast {
                action_complete = true;
                match action.target {
                    OrderTarget::Unit(target_entity) => {
                        if let Ok(target_unit) = unit_query.get(target_entity) {
                            sound_3d_events.send(Sound3dEvent::new(asset_loader.sounds.get_handle("8bit/swipe"), target_unit.coord().as_vec3(), 0.5));
                        }
                        health_events.send(HealthEvent::Sub(target_entity, 1));
                    }
                    OrderTarget::Voxel(coord_selection) => {

                    }
                }
            }
        }

        if action_complete {
            actioner.action_queue.pop_front();
        }
    }
}