//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitVelocityPlugin;
impl Plugin for UnitVelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sys_update_unit_velocity.in_schedule(CoreSchedule::FixedUpdate));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct UnitVelocity {
    speed: f32,
    direction: IVec3,
}

impl Default for UnitVelocity {
    fn default() -> Self {
        Self {
            speed: 0.0,
            direction: IVec3::ZERO,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_unit_velocity(
    mut unit_query: Query<(&mut Unit)>,
    game_time: Res<GameTime>,
) {
    
}