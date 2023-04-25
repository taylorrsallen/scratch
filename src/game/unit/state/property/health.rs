//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthEvent>()
            .add_event::<HealthNotify>()
            .add_system(evsys_health_events);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct Health {
    val: i32,
    max: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self { val: 10, max: 10 }
    }
}

impl Health {
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
pub enum HealthEvent {
    Add(Entity, u32),
    Sub(Entity, u32),
}

pub enum HealthNotify {
    Neg(Entity),
    Max(Entity),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_health_events(
    mut events: EventReader<HealthEvent>,
    mut notifies: EventWriter<HealthNotify>,
    mut health_query: Query<&mut Health>,
) {
    for event in events.iter() {
        match event {
            HealthEvent::Add(entity, val) => {
                if let Ok(mut health) = health_query.get_mut(*entity) {
                    health.val += *val as i32;
                    if health.val > health.max {
                        health.val = health.max;
                        notifies.send(HealthNotify::Max(*entity));
                    }
                }
            }
            HealthEvent::Sub(entity, val) => {
                if let Ok(mut health) = health_query.get_mut(*entity) {
                    health.val -= *val as i32;
                    if health.val <= 0 {
                        notifies.send(HealthNotify::Neg(*entity));
                    }
                }
            }
        }
    }
}