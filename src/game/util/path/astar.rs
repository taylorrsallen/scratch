//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use pathfinding::prelude::astar;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct AStarNode;
impl AStarNode {
    pub fn distance(
        a: &IVec3,
        b: &IVec3,
    ) -> u32 {
        let distance_coord = (*a - *b).abs();
        (distance_coord.x + distance_coord.y * 2 + distance_coord.z) as u32
    }

    fn append_walk_successors(
        node: &IVec3,
        unit: &Unit,
        successors: &mut Vec<(IVec3, u32)>,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        state_query: &Query<&UnitState>,
        defs: &Res<Defs>,
    ) {
        let side_neighbors = voxels.get_adjacent_side_values(node);
        for (side_face, side_voxel) in side_neighbors.iter().enumerate() {
            let side_coord = *node + VOXEL_SIDE_FACE_CHECKS[side_face];
            if side_voxel.is_walkable(&side_coord, voxels, defs) && entities.is_empty(&side_coord) {
                successors.push((side_coord, 1));
            }
        }
    }

    fn append_jump_successors(
        node: &IVec3,
        unit: &Unit,
        max_jump: u8,
        successors: &mut Vec<(IVec3, u32)>,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        state_query: &Query<&UnitState>,
        defs: &Res<Defs>,
    ) {
        for jump in 0..max_jump {
            let jump_coord = *node + (IVec3::Y + IVec3::Y * jump as i32);

            if voxels.get_value(&jump_coord).is_blocked(defs) { break; }
            if !entities.is_empty(&jump_coord) { break; }

            successors.push((jump_coord, 2));
            AStarNode::append_walk_successors(&jump_coord, unit, successors, voxels, entities, state_query, defs);
        }
    }

    fn append_fall_successors(
        node: &IVec3,
        unit: &Unit,
        max_fall: u8,
        successors: &mut Vec<(IVec3, u32)>,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        state_query: &Query<&UnitState>,
        defs: &Res<Defs>,
    ) {
        let side_neighbors = voxels.get_adjacent_side_values(node);
        for (side_face, side_neighbor) in side_neighbors.iter().enumerate() {
            let side_coord = *node + VOXEL_SIDE_FACE_CHECKS[side_face];
            if !side_neighbor.is_blocked(defs) && entities.is_empty(&side_coord) {
                successors.push((side_coord, 1));

                for fall in 0..max_fall {
                    let fall_coord = side_coord - (IVec3::Y + IVec3::Y * fall as i32);
                    if !voxels.get_value(&fall_coord).is_blocked(defs) && entities.is_empty(&fall_coord) {
                        successors.push((fall_coord, 2));
                    }
                }
            }
        }
    }

    fn ground_successors(
        node: &IVec3,
        unit: &Unit,
        max_jump: u8,
        max_fall: u8,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        state_query: &Query<&UnitState>,
        defs: &Res<Defs>,
    ) -> Vec<(IVec3, u32)> {
        let mut successors: Vec<(IVec3, u32)> = vec![];

        // Fine grained pathing should only be used for the leaf they're in (8*8*8) and the border voxels entering into other leaves
        // So a total pathing range of 10*10*10
        // The ideal algorithm for pathing might be to have the unit path to a midpoint, and then the target path to that midpoint, then combine the results
        // This way if either the unit or the endpoint is stuck we're covering half the range to figure it out
        if AStarNode::distance(node, unit.coord()) > 32 { return successors; }

        if voxels.adjacent_value_from_direction(node, GridDirection::Bottom).is_blocked(defs) {
            AStarNode::append_jump_successors(node, unit, max_jump, &mut successors, voxels, entities, state_query, defs);
            AStarNode::append_fall_successors(node, unit, max_fall, &mut successors, voxels, entities, state_query, defs);
        }

        successors
    }
}

pub struct AStar;
impl AStar {
    pub fn get_ground_path(
        unit: &Unit,
        max_jump: u8,
        max_fall: u8,
        target: &IVec3,
        voxels: &mut Accessor<Voxel>,
        entities: &mut Accessor<Option<Entity>>,
        state_query: &Query<&UnitState>,
        defs: &Res<Defs>,
    ) -> Option<(Vec<IVec3>, u32)> {
        if !voxels.get_value(target).is_walkable(target, voxels, defs) { return None; println!("target not walkable"); }
        if entities.get_value(target).is_some() { return None; println!("entity in target"); }
        astar(
            unit.coord(),
            |node| AStarNode::ground_successors(&node, unit, max_jump, max_fall, voxels, entities, state_query, defs),
            |node| AStarNode::distance(node, target),
            |node| *node == *target
        )
    }
}