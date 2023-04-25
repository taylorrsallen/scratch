//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

mod actioner;
pub use actioner::*;
mod body;
pub use body::*;
mod controller;
pub use controller::*;
mod state;
pub use state::*;
mod mover;
pub use mover::*;
mod orderable;
pub use orderable::*;
mod selectable;
pub use selectable::*;
mod velocity;
pub use velocity::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UnitActionerPlugin)
            .add_plugin(UnitBodyPlugin)
            .add_plugin(UnitControllerPlugin)
            .add_plugin(UnitStatePlugin)
            .add_plugin(UnitMoverPlugin)
            .add_plugin(UnitOrderablePlugin)
            .add_plugin(UnitSelectablePlugin)
            .add_plugin(UnitVelocityPlugin)
            .add_systems((
                    evsys_health_notify,
                    sys_sync_unit_transform,
                ).chain())
            .add_system(evsys_health_notifys.in_base_set(CoreSet::Last));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
const UNIT_STATE_BUSY_MASK: u16 = UnitState::Moving as u16 | UnitState::Acting as u16;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
//================================-================================-================================ 
// ActorBundle
/// A unit that generates and executes interactions.
#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    velocity: UnitVelocity,
    rigidbody: RigidBody,
    collider: Collider,
    transform: Transform,
}

impl Default for UnitBundle {
    fn default() -> Self {
        Self {
            unit: Unit::from_coord(&IVec3::ZERO),
            velocity: UnitVelocity::default(),
            rigidbody: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            transform: Transform::from_translation(Vec3::ZERO),
        }
    }
}

impl UnitBundle {
    pub fn new(
        coord: &IVec3,
        facing: &IVec3,
        scale: &IVec3,
    ) -> Self {
        Self {
            unit: Unit::new(coord, facing, scale),
            collider: Collider::cuboid(scale.x as f32 * 0.5, scale.y as f32 * 0.5, scale.z as f32 * 0.5),
            transform: Transform::from_translation(coord.as_vec3()),
            ..default()
        }
    }

    pub fn from_coord(
        coord: &IVec3,
    ) -> Self {
        Self {
            unit: Unit::from_coord(coord),
            transform: Transform::from_translation(coord.as_vec3()),
            ..default()
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitState {
    None   = 0x00,
    Moving = 0x01,
    Acting = 0x02,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitFaction {
    None   = 0x00,
    Player = 0x01,



    Evil   = 0x80,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
/// An entity which has coord, scale & facing on the grid.
/// 
/// Set coord & facing, & entity transform will be synced.
#[derive(Component)]
pub struct Unit {
    coord: IVec3,
    scale: IVec3,
    facing: IVec3,
    state: u32,
}

impl Unit {
    pub fn new(coord: &IVec3, scale: &IVec3, facing: &IVec3) -> Self {
        Self { coord: *coord, scale: *scale, facing: *facing, state: 0 }
    }
    
    pub fn from_coord(coord: &IVec3) -> Self {
        Self { coord: *coord, scale: IVec3::ONE, facing: IVec3::Z, state: 0 }
    }

    //-------------------------------- -------------------------------- --------------------------------
    // PROPERTIES
    pub fn coord(&self) -> &IVec3 { &self.coord }
    pub fn scale(&self) -> &IVec3 { &self.scale }
    pub fn facing(&self) -> &IVec3 { &self.facing }
    pub fn state(&self) -> u32 { self.state }

    //-------------------------------- -------------------------------- --------------------------------
    // GET SET
    pub fn try_get_unit_entity(
        entity: &Entity,
        unit_query: &Query<&Unit>,
    ) -> Option<Entity> {
        if let Ok(_) = unit_query.get(*entity) {
            Some(*entity)
        } else {
            None
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_health_notify(
    mut commands: Commands,
    mut notifies: EventReader<HealthNotify>,
    mut sound_3d_events: EventWriter<Sound3dEvent>,
    level_tree_query: Query<&EntityTree, With<LevelTree>>,
    unit_query: Query<&Unit>,
    asset_loader: Res<AssetLoader>,
) {
    let mut entities = level_tree_query.single().get_accessor();
    for notify in notifies.iter() {
        match notify {
            HealthNotify::Neg(entity) => {
                if let Ok(unit) = unit_query.get(*entity) {
                    // sound_3d_events.send(Sound3dEvent::new(asset_loader.sounds.get_handle("8bit/explosion"), unit.coord().as_vec3(), 1.0));
                    entities.set_value_off(unit.coord());
                }
                commands.entity(*entity).despawn_recursive();
            }
            HealthNotify::Max(entity) => {
                
            }
        }
    }
}

fn evsys_health_notifys(
    mut notifies: EventReader<HealthNotify>,
    mut sound_3d_events: EventWriter<Sound3dEvent>,
    unit_query: Query<&Unit>,
    asset_loader: Res<AssetLoader>,
) {
    for notify in notifies.iter() {
        match notify {
            HealthNotify::Neg(entity) => {
                sound_3d_events.send(Sound3dEvent::new(asset_loader.sounds.get_handle("8bit/explosion"), Vec3::ZERO, 1.0));
                if let Ok(unit) = unit_query.get(*entity) {
                }
            }
            HealthNotify::Max(entity) => {
                
            }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
/// Read: `Unit` >> Write: `Transform`
fn sys_sync_unit_transform(
    mut unit_query: Query<(&mut Transform, &Unit)>,
    time: Res<Time>,
) {
    for (mut transform, unit) in unit_query.iter_mut() {
        let anim_speed = 45.0;
        let direction = unit.coord.as_vec3() - transform.translation;
        let distance = direction.length();
        let delta = time.delta_seconds();
        let move_vec = direction.normalize_or_zero() * anim_speed;

        if move_vec.length() * delta > distance {
            transform.translation = unit.coord.as_vec3();
        } else {
            transform.translation += move_vec * delta;
        }
        
        transform.look_to(unit.facing.as_vec3(), Vec3::Y);
    }
}