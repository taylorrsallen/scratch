//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct OrdererPlugin;
impl Plugin for OrdererPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrderEvent>()
        .add_systems((
                evsys_receive_order_events,
            ).in_base_set(PlayerSet::Actions));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct PlayerOrderer {
    order_kind: Option<OrderKind>,
}

impl Default for PlayerOrderer {
    fn default() -> Self {
        Self { order_kind: None }
    }
}

impl PlayerOrderer {
    pub fn get_order_kind(&self) -> &Option<OrderKind> {
        &self.order_kind
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub enum OrderEventKind {
    Unit(Entity),
    Voxel(CoordSelection),
    StartPath(CoordSelection),
    DragPath(CoordSelection),
}

pub struct OrderEvent {
    sender: Entity,
    kind: OrderEventKind,
}

impl OrderEvent {
    pub fn new(sender: Entity, kind: OrderEventKind) -> Self {
        Self { sender, kind }
    }

    pub fn unit(sender: Entity, select: Entity) -> Self {
        Self { sender, kind: OrderEventKind::Unit(select) }
    }

    pub fn voxel(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: OrderEventKind::Voxel(select) }
    }

    pub fn start_path(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: OrderEventKind::StartPath(select) }
    }

    pub fn drag_path(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: OrderEventKind::DragPath(select) }
    }

    pub fn get_unit_target_order_action(unit_entity: &Entity, orderer: &PlayerOrderer, unit_query: &Query<&Unit>) -> OrderKind {
        if let Some(order) = orderer.get_order_kind() {
            *order
        } else {
            // get a faction mask with on bits for factions that every unit in selection has
            // decide whether to attack or follow based on that faction mask against target units faction flags
            OrderKind::Action(ActionOrderKind::Attack)
        }
    }

    pub fn get_voxel_target_order_action(orderer: &PlayerOrderer) -> OrderKind {
        if let Some(order) = orderer.get_order_kind() {
            *order
        } else {
            OrderKind::Move
        }
    }

    pub fn set_orders(
        order: &Order,
        selection: &PlayerSelector,
        orderable_query: &mut Query<&mut UnitOrderable, With<PlayerControlled>>,
    ) {
        for entity in selection.selected_units().iter() {
            let mut orderable = if let Ok(mut orderable) = orderable_query.get_mut(*entity) { orderable } else { continue; };
            orderable.clear_orders();
            orderable.queue_order(order);
        }
    }

    pub fn queue_orders(
        order: &Order,
        selection: &PlayerSelector,
        orderable_query: &mut Query<&mut UnitOrderable, With<PlayerControlled>>,
    ) {
        for entity in selection.selected_units().iter() {
            let mut orderable = if let Ok(mut orderable) = orderable_query.get_mut(*entity) { orderable } else { continue; };
            orderable.queue_order(order);
        }
    }

    pub fn order(
        &self,
        orderer: &PlayerOrderer,
        selector: &PlayerSelector,
        marker_events: &mut EventWriter<MarkerEvent>,
        sound_2d_events: &mut EventWriter<Sound2dEvent>,
        orderable_query: &mut Query<&mut UnitOrderable, With<PlayerControlled>>,
        unit_query: &Query<&Unit>,
        input_state: &Res<InputState>,
    ) {
        match self.kind {
            OrderEventKind::Unit(unit_entity) => {
                let order = Order::new(OrderEvent::get_unit_target_order_action(&unit_entity, orderer, unit_query), OrderTarget::Unit(unit_entity));
                println!("{:?}", order);
                if !input_state.pressed(InputAction::MultiMod) {
                    OrderEvent::set_orders(&order, selector, orderable_query);
                } else {
                    OrderEvent::queue_orders(&order, selector, orderable_query);
                }
            }
            OrderEventKind::Voxel(select) => {
                let order = Order::new(OrderEvent::get_voxel_target_order_action(orderer), OrderTarget::Voxel(select));
                println!("{:?}", order);
                if !input_state.pressed(InputAction::MultiMod) {
                    OrderEvent::set_orders(&order, selector, orderable_query);
                } else {
                    OrderEvent::queue_orders(&order, selector, orderable_query);
                }
            }
            OrderEventKind::StartPath(select) => { /* println!("StartSelection"); */ }
            OrderEventKind::DragPath(select) => { /* println!("DragSelection"); */ }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_receive_order_events(
    mut order_events: EventReader<OrderEvent>,
    mut marker_events: EventWriter<MarkerEvent>,
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    mut orderable_query: Query<&mut UnitOrderable, With<PlayerControlled>>,
    player_query: Query<(&PlayerOrderer, &PlayerSelector)>,
    unit_query: Query<&Unit>,
    input_state: Res<InputState>,
) {
    for order_event in order_events.iter() {
        if let Ok((orderer, selection)) = player_query.get(order_event.sender) {
            order_event.order(&orderer, &selection, &mut marker_events, &mut sound_2d_events, &mut orderable_query, &unit_query, &input_state);
        }
    }
}