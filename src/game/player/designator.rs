//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct DesignatePlugin;
impl Plugin for DesignatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DesignateEvent>()
            .add_systems((
                    evsys_receive_designate_events,
                    dumb_system,
                ).in_base_set(PlayerSet::Actions));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct DesignatorWarning(Entity);

#[derive(Component)]
pub struct PlayerDesignator {
    ability: Option<u32>,
    units: Option<Vec<Entity>>,
    voxels: Option<Vec<CoordSelection>>,
}

impl Default for PlayerDesignator {
    fn default() -> Self {
        Self { ability: None, units: None, voxels: None }
    }
}

impl PlayerDesignator {
    pub fn is_designating(&self) -> bool {
        self.ability.is_some()
    }

    pub fn designate_unit(&mut self, unit: &Entity, marker_events: &mut EventWriter<MarkerEvent>) {
        println!("Designate Unit");
        if let Some(units) = self.units.as_mut() {
            units.push(*unit);
        } else {
            self.units = Some(vec![*unit]);
        }
    }

    pub fn designate_voxel(&mut self, voxel: &CoordSelection, marker_events: &mut EventWriter<MarkerEvent>) {
        println!("Designate Voxel");
        if let Some(voxels) = self.voxels.as_mut() {
            voxels.push(*voxel);
        } else {
            self.voxels = Some(vec![*voxel]);
        }
    }

    pub fn undesignate_all(&mut self, marker_events: &mut EventWriter<MarkerEvent>) {
        self.units = None;
        self.voxels = None;
    }

    pub fn restart(&mut self, id: u32, marker_events: &mut EventWriter<MarkerEvent>) {
        println!("Restart Designation");
        self.undesignate_all(marker_events);
        self.ability = Some(id);
    }
    
    pub fn cancel(&mut self, marker_events: &mut EventWriter<MarkerEvent>) {
        println!("Cancel Designation");
        self.undesignate_all(marker_events);
        self.ability = None;
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub enum DesignateKind {
    Restart(u32),
    Cancel,
    Unit(Entity),
    Voxel(CoordSelection),
    StartSelection(CoordSelection),
    DragSelection(CoordSelection),
}

pub struct DesignateEvent {
    sender: Entity,
    kind: DesignateKind,
}

impl DesignateEvent {
    pub fn new(sender: Entity, kind: DesignateKind) -> Self {
        Self { sender, kind }
    }

    pub fn restart(sender: Entity, id: u32) -> Self {
        Self { sender, kind: DesignateKind::Restart(id) }
    }

    pub fn cancel(sender: Entity) -> Self {
        Self { sender, kind: DesignateKind::Cancel }
    }

    pub fn unit(sender: Entity, select: Entity) -> Self {
        Self { sender, kind: DesignateKind::Unit(select) }
    }

    pub fn voxel(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: DesignateKind::Voxel(select) }
    }

    pub fn start_selection(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: DesignateKind::StartSelection(select) }
    }

    pub fn drag_selection(sender: Entity, select: CoordSelection) -> Self {
        Self { sender, kind: DesignateKind::DragSelection(select) }
    }

    pub fn designate(
        &self,
        designator: &mut PlayerDesignator,
        marker_events: &mut EventWriter<MarkerEvent>,
        sound_2d_events: &mut EventWriter<Sound2dEvent>,
    ) {
        match self.kind {
            DesignateKind::Restart(id) => { designator.restart(id, marker_events); }
            DesignateKind::Cancel => { designator.cancel(marker_events); }
            DesignateKind::Unit(entity) => { designator.designate_unit(&entity, marker_events); }
            DesignateKind::Voxel(select) => { designator.designate_voxel(&select, marker_events); }
            DesignateKind::StartSelection(select) => { /* println!("StartSelection"); */ }
            DesignateKind::DragSelection(select) => { /* println!("DragSelection"); */ }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_receive_designate_events(
    mut commands: Commands,
    mut designate_events: EventReader<DesignateEvent>,
    mut marker_events: EventWriter<MarkerEvent>,
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    mut designator_query: Query<(Entity, &mut PlayerDesignator)>,
    input_state: Res<InputState>,
    asset_loader: Res<AssetLoader>,
) {
    for designate_event in designate_events.iter() {
        if let Ok((entity, mut designator)) = designator_query.get_mut(designate_event.sender) {
            if designator.is_designating() {
                commands.spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Percent(33.0),
                                bottom: Val::Percent(10.0),
                                ..default()
                            },
                            ..default()
                        },
                        text: Text::from_section(
                            "Right Click to cancel Non-Existant Ability",
                            TextStyle {
                                font: asset_loader.fonts.get_handle("hack/italic"),
                                font_size: 26.0,
                                color: Color::WHITE,
                            }
                        ),
                        ..default()
                    })
                    .insert(DesignatorWarning(entity));
            }
            // TODO: Some abilities might have multi designation as the default
            if !input_state.pressed(InputAction::MultiMod) { designator.undesignate_all(&mut marker_events); }
            designate_event.designate(&mut designator, &mut marker_events, &mut sound_2d_events);
        }
    }
}

fn dumb_system(
    mut commands: Commands,
    designator_query: Query<(&PlayerDesignator)>,
    dumb_query: Query<(Entity, &DesignatorWarning)>,
) {
    for (warning_entity, warning) in dumb_query.iter() {
        if let Ok(designator) = designator_query.get(warning.0) {
            if !designator.is_designating() {
                commands.entity(warning_entity).despawn_recursive();
            }
        }
    }
}

// You're designating an ability
// Cast Time:
//   Show time to cast beside user
    // Read cast time from an ability and update text with it
// Direct:
//   Highlight user and target
    // Read AbilityKind::Direct from ability and update STATE
// Line Projectile:
//   Draw line from user to target
    // Read AbilityEffect::Projectile and ProjectilePath::Line and update STATE
// Gravity Projectile:
//   Draw arc from user to target
    // Read AbilityEffect::Projectile and ProjectilePath::Gravity and update STATE
// Accuracy Projectile:
//   Show accuracy cone and % chances to hit targets in cone
    // Read AbilityEffect::Projectile and ProjectileProperties::Accuracy and update STATE
// Movement Ability:
//   Highlight tile path from user to target
    // Read AbilityKind::Movement and update STATE
// Melee:
//   Highlight valid tiles in range around user
//   Highlight tiles affected by attack while hovering over valid tiles
    // Read AbilityEffect::Melee and update STATE
// Area:
//   Highlight tiles affected by Area around target
    // Read AbilityEffect::Area and update STATE

// Fire a bow:
    // AbilityCast::Cast
    // AbilitySource::User
    // AbilityTarget::Single
    // AbilityRange::None
    // AbilityForm::Projectile
        // Projectile AbilityTarget::Single
        // Projectile AbilityForm::Hitbox
        // ProjectileProperties::
            // Physical
            // Gravity
            // Weight

pub struct Ability {
    cast: AbilityCast,
    source: AbilitySource,
    target_shape: AbilityTargetShape,
    target_kind: AbilityTargetKind,
    range: AbilityRange,
    form: AbilityEffect,
}

/// * Could change with stats:
///     * Faster cast with better magic
///     * Faster swing with more strength
///     * Faster move with lower weight
/// * Could change based on input:
///     * Quick melee vs. heavy melee
pub enum AbilityCast {
    /// Activates instantly
    Instant,

    /// Activates after a delay
    Cast,

    /// Remains active for a time
    Channel,
}

/// * Could change based on input:
///     * Changing the heigh meteor will be generated at
pub enum AbilitySource {
    None,

    /// Effect originates from user
    /// * Bow: Source is users bow
    /// * Melee: Source is users weapon
    User,

    /// Effect originates from an offset to the user
    /// * Meteor: Source is above user
    UserOffset,

    /// Effect originates from target
    /// * Flame Pillar source is from beneath its target
    Target,
}

/// * Could change based on input:
///     * Changing the form of a spell
///     * Changing the attack type for a melee weapon
///         * stab vs. swing with a spear
pub enum AbilityTargetShape {
    /// Single coord or unit
    Single,

    /// Line of coords and/or units
    Line,

    /// Area of coords and/or units
    Area,

    /// Arbitrary set of coords and/or units
    Set,
}

/// Valid targets for ability
/// * Could change based on input:
///     * Changing a spell to only affect hostiles
pub enum AbilityTargetKind {
    /// * Passive effects
    /// * Buff spells
    User      = 0x01,

    /// * Gate: Teleport an ally
    Alignment = 0x02,
    
    /// * Holy: Purge undead
    Species   = 0x04,

    /// For instance, mental spells cannot target tiles
    Tile    = 0x10,
}

/// * Could change with stats:
///     * Further Lifesap range with better magic
/// * Could change based on input:
///     * Using a contact form of a spell
pub enum AbilityRange {
    /// Can (try to) target anything
    None,

    /// Target things the user can touch
    Touch,

    /// Target within an area around User
    Area,

    /// Target anything in sight of User
    Sight,
}

pub enum AbilityEffect {
    /// Instantly affects the target when cast
    /// * Heal: Instantly apply heal
    /// * Regen: Instantly apply regen buff
    /// * Lifesap: Instantly steal hp
    /// * Cheer: Instantly apply damage/stamina buff
    Direct,

    /// Creates hitbox at source with ability target as hitbox target
    /// * Melee: Swing/stab with hitbox
    /// * Grab: Reach out with hitbox
    /// * Flame Pillar: Cylinder hitbox with burn damage
    Hitbox,

    /// Creates a projectile at source aimed at target
    /// * Bow: Fire arrow projectile from user to target
    /// * Fireball: Fire flame projectile from user to target
    /// * Meteor: Summon flame projectile from above user to target
    Projectile,
}
pub enum AbilityKind {

}