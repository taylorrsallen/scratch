//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct QuickslotsPlugin;
impl Plugin for QuickslotsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QuickslotEvent>()
            .add_system(onsys_spawn_quickslots.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(onsys_spawn_quickslots.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems((
                sys_update_quickslots_state,
                sys_update_quickslots,
            ).chain().in_base_set(PlayerSet::Input));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct Quickslot {
    ability: Option<Ability>,
    binding: InputBinding,
    ability_id: u32,
}

impl Quickslot {
    pub fn new(ability: Option<Ability>, binding: InputBinding, ability_id: u32) -> Self {
        Self { ability, binding, ability_id }
    }

    pub fn new_empty(binding: InputBinding, ability_id: u32) -> Self {
        Self { ability: None, binding, ability_id }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub enum QuickslotEvent {
    Select(Entity, u8),
    Deselect(Entity),
    ToggleSelect(Entity, u8),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct Quickslots {
    pub slots: Vec<Quickslot>,

    just_released: Bitmask,
    just_pressed: Bitmask,
    pressed: Bitmask,

    activate_sound: Option<Sound2dEvent>,
    deactivate_sound: Option<Sound2dEvent>,
}

impl Quickslots {
    pub fn new(select_sound: Option<Sound2dEvent>, deselect_sound: Option<Sound2dEvent>) -> Self {
        let mut quickslots = Self {
            slots: vec![],

            just_released: Bitmask::from_cube_log2dim(3, false),
            just_pressed: Bitmask::from_cube_log2dim(3, false),
            pressed: Bitmask::from_cube_log2dim(3, false),

            activate_sound: select_sound,
            deactivate_sound: deselect_sound
        };

        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key1]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key2]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key3]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key4]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key5]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key6]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key7]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key8]), 0));
        quickslots.push_slot(Quickslot::new_empty(InputBinding::any().with_keys(vec![KeyCode::Key9]), 0));
        quickslots
    }

    pub fn push_slot(&mut self, slot: Quickslot) {
        self.slots.push(slot);
    }

    pub fn set_slot(&mut self, slot: Quickslot, index: u8) {
        if self.slots.get(index as usize).is_some() { self.slots[index as usize] = slot; }
    }

    pub fn remove_slot(&mut self, index: u8) {
        if self.slots.get(index as usize).is_some() { self.slots.swap_remove(index as usize); }
    }
    
    fn play_deactivate_sound(&self, sound_2d_events: &mut EventWriter<Sound2dEvent>) {
        if let Some(sound_2d_event) = &self.activate_sound { sound_2d_events.send(sound_2d_event.clone()); }
    }
    
    fn play_activate_sound(&self, sound_2d_events: &mut EventWriter<Sound2dEvent>) {
        if let Some(sound_2d_event) = &self.deactivate_sound { sound_2d_events.send(sound_2d_event.clone()); }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STATE SYSTEMS
fn onsys_spawn_quickslots(
    
) {

}

fn onsys_despawn_quickslots(
    
) {

}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_receive_quickslot_events(
    mut quickslot_events: EventReader<QuickslotEvent>,
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    mut quickslots_query: Query<&mut Quickslots>,
) {
    for quickslot_event in quickslot_events.iter() {
        // match quickslot_event {
        //     QuickslotEvent::Select(entity, index) => {
        //         if let Ok(mut quickslots) = quickslots_query.get_mut(*entity) {
        //             quickslots.activate_slot(*index, &mut sound_2d_events);
        //         }
        //     }
        //     QuickslotEvent::Deselect(entity) => {
        //         if let Ok(mut quickslots) = quickslots_query.get_mut(*entity) {
        //             quickslots.deactivate_slot(&mut sound_2d_events);
        //         }
        //     }
        //     QuickslotEvent::ToggleSelect(entity, index) => {
        //         if let Ok(mut quickslots) = quickslots_query.get_mut(*entity) {
        //             quickslots.toggle_slot(*index, &mut sound_2d_events);
        //         }
        //     }
        // }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_quickslots_state(
    mut quickslots_query: Query<(&mut Quickslots)>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    for (mut quickslots) in quickslots_query.iter_mut() {
        let mut quickslot_states: Vec<(u8, u8)> = vec![];
        for (index, quickslot) in quickslots.slots.iter().enumerate() {
            let state = quickslot.binding.state(&keyboard, &mouse);
            if state != 0 { quickslot_states.push((state, index as u8)); }
        }

        quickslots.just_released.set_off();
        quickslots.just_pressed.set_off();
        quickslots.pressed.set_off();
    
        for (state, index) in quickslot_states {
            if is_flag_on_u8(state, InputStateFlag::JustReleased) { quickslots.just_released.set_bit_on(index as usize); }
            if is_flag_on_u8(state, InputStateFlag::JustPressed)  { quickslots.just_pressed.set_bit_on(index as usize); }
            if is_flag_on_u8(state, InputStateFlag::Pressed)      { quickslots.pressed.set_bit_on(index as usize); }
        }
    }
}

fn sys_update_quickslots(
    mut designate_events: EventWriter<DesignateEvent>,
    mut quickslot_events: EventWriter<QuickslotEvent>,
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    mut quickslots_query: Query<(Entity, &mut Quickslots)>,
) {
    for (entity, mut quickslots) in quickslots_query.iter_mut() {
        for i in OnMaskIter::new(0, &quickslots.just_pressed) {
            let quickslot = &quickslots.slots[i];
            quickslot_events.send(QuickslotEvent::Select(entity, i as u8));
            designate_events.send(DesignateEvent::restart(entity, quickslot.ability_id));
            quickslots.play_activate_sound(&mut sound_2d_events);
            // if let Some(ability) = quickslot.ability {

            // }
            // match quickslot.activation {
            //     QuickslotActivation::Instant => {
            //         // Use the... ability?
            //         instants.push(i as u8);
            //     }
            //     QuickslotActivation::Toggle => {}
            //     QuickslotActivation::Hold => {}
            // }
        }
    }
}