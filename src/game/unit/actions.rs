//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
/// Update: {Read: `CommandData` >> Update: `GameData`}
/// 
/// `RawInput`: Keyboard/Mouse etc.
/// 
/// `PlayerState`: Any player data that does not interact with the game world. Player selections, in_combat, camera_position, paused, etc.
/// 
/// `LevelState`: Things that exist in the level. Voxels, positions of entities, etc.
/// 
/// `Tasks`: Have a priority level and a condition which must be fulfilled: MoveToCoord, FollowUnit, KillUnit, PickupItem, GuardArea, Roam, RoamArea, etc.
/// 
/// `Actions`: Are queued, and describe changes to be carried out on the game state: MoveTo, Attack, Use
/// 
/// Full Unit Update Process:
/// 1. [[CoreSet::PreUpdate]] `PlayerControlled` OR `AIControlled`
///     * UpdatePlayerState System: Read `RawInput` && `LevelState` >> Write `PlayerState`
///         * (`LeftClick` && raycast into level used to fill `selected_units: Vec<Unit>`)
///     * UpdateTasks System (Player): Read `RawInput` && `PlayerState` >> Write `Tasks, With<Player>`
///         * (`RightClick` + `selected_units` writes `Tasks` to `UnitMind`)
///     * UpdateTasks System (AI): Read `AIState` >> Write `Tasks, With<AI>`
/// 2. [[CoreSet::PreUpdate]] `UnitMind`
///     * UpdateActions System: Read `Tasks` && `LevelState` >> Write `Actions` 
///         1. Iterate through tasks
///         2. Check if the task can be solved with the actions the unit is capable of performing
///         3. If the task is possible, find the best action and assign it
///             1. If the task is not possible, but it could be (I want to walk there, but now someone is standing there), then try and update it
///                 1. If the task is updated successfully, find the best action and assign it
///                 2. If the update is unsuccessful, lower the tasks priority
///             2. If the task is impossible (I want to attack `Unit` but I have no arms), remove it
/// 3. [[CoreSet::Update]] `UnitActions`
///     1. ExecuteActions System: Read `LevelState` && `Actions` && `Tasks` >> Write `LevelState` && `Tasks`
///         1. Iterate through actions
///         2. Read level to make sure action is possible, then write action to level, remove the action whether it is possible or not
///         3. If action was completed, check if associated task condition is met, and if it is, remove the task
/// 
/// Notes:
///     * When the game is paused:
///         * player state will read all player units with tasks assigned
///         * get steps until the next player unit will have no task
///         * set steps_until_pause to that amount
fn sys_execute_actions(
    mut actions_query: Query<(&mut UnitActions)>,
    level_tree_query: Query<(&VoxelTree, &EntityTree), With<LevelTree>>,
    defs: Res<Defs>,
) {
    let (voxel_tree, entity_tree) = level_tree_query.single();
    let mut voxels = voxel_tree.get_accessor();
    let mut entities = entity_tree.get_accessor();
    for (mut actions) in actions_query.iter_mut() {
        actions.execute(&mut voxels, &mut entities, &mut defs);
    }
}

// pub fn apply_next_coord(
//     &mut self,
//     unit_entity: &Option<Entity>,
//     entities: &mut Accessor<Option<Entity>>,
// ) {
//     if let Some(next_coord) = self.desired_coord {
//         if entities.get_value(&next_coord).is_none() {
//             entities.set_value_off(&self.coord);
//             entities.set_value_on(&next_coord, unit_entity);
//             self.coord = next_coord;
//             self.desired_coord = None;
//         }
//     }
// }