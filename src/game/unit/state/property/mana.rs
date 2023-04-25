//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct ManaPlugin;
impl Plugin for ManaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ManaEvent>()
            .add_event::<ManaNotify>()
            .add_system(evsys_mana_events);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct Mana {
    val: i32,
    max: i32,
}

impl Default for Mana {
    fn default() -> Self {
        Self { val: 10, max: 10 }
    }
}

impl Mana {
    pub fn new(val: i32, max: i32) -> Self {
        Self { val, max }
    }

    pub fn percent(&self) -> f32 {
        if self.val > 0 && self.max > 0 {
            self.val as f32 / self.max as f32
        } else {
            0.0
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub enum ManaEvent {
    Add(Entity, u32),
    Sub(Entity, u32),
}

pub enum ManaNotify {
    Neg,
    Max,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_mana_events(
    mut events: EventReader<ManaEvent>,
    mut notifies: EventWriter<ManaNotify>,
    mut mana_query: Query<&mut Mana>,
) {
    for event in events.iter() {
        match event {
            ManaEvent::Add(entity, val) => {
                if let Ok(mut mana) = mana_query.get_mut(*entity) {
                    mana.val += *val as i32;
                    if mana.val > mana.max {
                        mana.val = mana.max;
                        notifies.send(ManaNotify::Max);
                    }
                }
            }
            ManaEvent::Sub(entity, val) => {
                if let Ok(mut mana) = mana_query.get_mut(*entity) {
                    mana.val -= *val as i32;
                    if mana.val <= 0 {
                        notifies.send(ManaNotify::Neg);
                    }
                }
            }
        }
    }
}