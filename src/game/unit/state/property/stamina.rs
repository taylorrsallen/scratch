//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct StaminaPlugin;
impl Plugin for StaminaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StaminaEvent>()
            .add_event::<StaminaNotify>()
            .add_system(evsys_stamina_events);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct Stamina {
    val: i32,
    max: i32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self { val: 10, max: 10 }
    }
}

impl Stamina {
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
pub enum StaminaEvent {
    Add(Entity, u32),
    Sub(Entity, u32),
}

pub enum StaminaNotify {
    Neg,
    Max,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_stamina_events(
    mut events: EventReader<StaminaEvent>,
    mut notifies: EventWriter<StaminaNotify>,
    mut stamina_query: Query<&mut Stamina>,
) {
    for event in events.iter() {
        match event {
            StaminaEvent::Add(entity, val) => {
                if let Ok(mut stamina) = stamina_query.get_mut(*entity) {
                    stamina.val += *val as i32;
                    if stamina.val > stamina.max {
                        stamina.val = stamina.max;
                        notifies.send(StaminaNotify::Max);
                    }
                }
            }
            StaminaEvent::Sub(entity, val) => {
                if let Ok(mut stamina) = stamina_query.get_mut(*entity) {
                    stamina.val -= *val as i32;
                    if stamina.val <= 0 {
                        notifies.send(StaminaNotify::Neg);
                    }
                }
            }
        }
    }
}