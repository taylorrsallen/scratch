//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use std::collections::VecDeque;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitOrderablePlugin;
impl Plugin for UnitOrderablePlugin {
    fn build(&self, app: &mut App) {}
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderKind {
    Move,
    Action(ActionOrderKind)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionOrderKind {
    Use,
    Attack,
    Ability(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderTarget {
    Unit(Entity),
    Voxel(CoordSelection),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    kind: OrderKind,
    target: OrderTarget,
}

impl Order {
    pub fn new(kind: OrderKind, target: OrderTarget) -> Self {
        Self { kind, target }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveOrder {
    target: OrderTarget,
}

impl From<&Order> for MoveOrder {
    fn from(order: &Order) -> Self {
        MoveOrder::new(order.target)
    }
}

impl MoveOrder {
    pub fn new(target: OrderTarget) -> Self {
        Self { target }
    }

    pub fn target(&self) -> &OrderTarget { &self.target }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionOrder {
    kind: ActionOrderKind,
    target: OrderTarget,
}

impl From<&Order> for ActionOrder {
    fn from(order: &Order) -> Self {
        ActionOrder::new(
            match order.kind {
                OrderKind::Action(action) => { action }
                _ => { panic!("Tried to get an action order from a move order!") }
            },
            order.target
        )
    }
}

impl ActionOrder {
    pub fn new(kind: ActionOrderKind, target: OrderTarget) -> Self {
        Self { kind, target }
    }

    pub fn kind(&self) -> &ActionOrderKind { &self.kind }
    pub fn target(&self) -> &OrderTarget { &self.target }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
/// Receives orders, but does nothing with them without UnitActions
/// 
/// Because Movement & Actions can occur at the same time, they're stored separately
#[derive(Component)]
pub struct UnitOrderable {
    move_orders: VecDeque<MoveOrder>,
    action_orders: VecDeque<ActionOrder>,
}

impl Default for UnitOrderable {
    fn default() -> Self {
        Self {
            move_orders: VecDeque::new(),
            action_orders: VecDeque::new(),
        }
    }
}

impl UnitOrderable {
    pub fn set_order(&mut self, order: &Order) {
        if order.kind == OrderKind::Move {
            self.clear_move_orders();
            self.move_orders.push_back(MoveOrder::from(order));
        } else {
            self.clear_action_orders();
            self.action_orders.push_back(ActionOrder::from(order));
        }
    }

    pub fn queue_order(&mut self, order: &Order) {
        if order.kind == OrderKind::Move {
            self.move_orders.push_back(MoveOrder::from(order));
        } else {
            self.action_orders.push_back(ActionOrder::from(order));
        }
    }

    pub fn queue_move_order(&mut self, order: &MoveOrder) {
        self.move_orders.push_back(*order);
    }

    pub fn queue_action_order(&mut self, order: &ActionOrder) {
        self.action_orders.push_back(*order);
    }

    pub fn clear_orders(&mut self) {
        self.move_orders.clear();
        self.action_orders.clear();
    }

    pub fn clear_move_orders(&mut self) {
        self.move_orders.clear();
    }

    pub fn clear_action_orders(&mut self) {
        self.action_orders.clear();
    }

    pub fn next_move_order(&self) -> Option<&MoveOrder> {
        self.move_orders.front()
    }

    pub fn next_action_order(&self) -> Option<&ActionOrder> {
        self.action_orders.front()
    }

    pub fn pop_move_order(&mut self) -> Option<MoveOrder> {
        self.move_orders.pop_front()
    }

    pub fn pop_action_order(&mut self) -> Option<ActionOrder> {
        self.action_orders.pop_front()
    }
}