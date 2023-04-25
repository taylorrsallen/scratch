//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use super::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct Accessor<ValueType: Clone + Copy + PartialEq + Eq> {
    root_node: Arc<RwLock<RootNode<ValueType>>>,
    background: ValueType,

    trunk_key: IVec3,
    trunk_node: Option<Arc<RwLock<TrunkNode<ValueType>>>>,
    branch_key: IVec3,
    branch_node: Option<Arc<RwLock<BranchNode<ValueType>>>>,
    leaf_key: IVec3,
    leaf_node: Option<Arc<RwLock<LeafNode<ValueType>>>>,
}

impl Accessor<Voxel> {
    pub fn set_remesh(
        &mut self,
        coord: &IVec3,
    ) {
        self.eval_leaf_cache(coord);
        
        let (branch_node, trunk_node) = self.root_node.write().unwrap().set_redraw_and_cache(coord);
        self.insert_branch(coord, &branch_node);
        self.insert_trunk(coord, &trunk_node);
    }

    pub fn set_voxel_off(
        &mut self,
        coord: &IVec3,
    ) {
        if let Some(leaf_node) = self.eval_leaf_cache(coord) {
            leaf_node.write().unwrap().set_value_at_coord(coord, &self.background, false);
        } else if let Some(branch_node) = self.eval_branch_cache(coord) {
            let leaf_node = branch_node.write().unwrap().set_value_and_cache(coord, &self.background, false);
            self.insert_leaf(coord, &leaf_node);
        } else if let Some(trunk_node) = self.eval_trunk_cache(coord) {
            let (leaf_node, branch_node) = trunk_node.write().unwrap().set_value_and_cache(coord, &self.background, false);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
        } else {
            let (leaf_node, branch_node, trunk_node) = self.root_node.write().unwrap().set_value_and_cache(coord, &self.background, false);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
            self.insert_trunk(coord, &trunk_node);
        }

        let neighbors = self.get_adjacent_values(coord);
        for face_coord in GRID_DIRECTIONS.iter().map(|face_coord| *face_coord + *coord) {
            self.set_remesh(&face_coord);
        }
    }
}

impl Accessor<Option<Entity>> {
    pub fn is_empty(
        &mut self,
        coord: &IVec3,
    ) -> bool {
        self.get_value(&coord).is_none()
    }
}

impl<ValueType: Clone + Copy + PartialEq + Eq> Accessor<ValueType> {
    pub fn new(
        root_node: &Arc<RwLock<RootNode<ValueType>>>,
    ) -> Self {
        let root_read_lock = root_node.read().unwrap();

        Self {
            root_node: root_node.clone(),
            background: root_read_lock.background,

            trunk_key: IVec3::new(i32::MAX, i32::MAX, i32::MAX),
            trunk_node: None,
            branch_key: IVec3::new(i32::MAX, i32::MAX, i32::MAX),
            branch_node: None,
            leaf_key: IVec3::new(i32::MAX, i32::MAX, i32::MAX),
            leaf_node: None,
        }
    }



    pub fn adjacent_value_from_index(
        &mut self,
        global_coord: &IVec3,
        face: usize,
    ) -> ValueType {
        self.get_value(&(*global_coord + GRID_DIRECTIONS[face]))
    }
    
    pub fn adjacent_value_from_direction(
        &mut self,
        global_coord: &IVec3,
        face: GridDirection,
    ) -> ValueType {
        self.get_value(&(*global_coord + GRID_DIRECTIONS[face as usize]))
    }

    pub fn get_adjacent_vertical_values(
        &mut self,
        global_coord: &IVec3,
    ) -> [ValueType; 2] {
        [
            self.adjacent_value_from_index(global_coord, 2),
            self.adjacent_value_from_index(global_coord, 3),
        ]
    }

    pub fn get_adjacent_side_values(
        &mut self,
        global_coord: &IVec3,
    ) -> [ValueType; 4] {
        [
            self.adjacent_value_from_index(global_coord, 0),
            self.adjacent_value_from_index(global_coord, 1),
            self.adjacent_value_from_index(global_coord, 4),
            self.adjacent_value_from_index(global_coord, 5),
        ]
    }

    pub fn get_adjacent_values(
        &mut self,
        global_coord: &IVec3,
    ) -> [ValueType; 6] {
        [
            self.adjacent_value_from_index(global_coord, 0),
            self.adjacent_value_from_index(global_coord, 1),
            self.adjacent_value_from_index(global_coord, 2),
            self.adjacent_value_from_index(global_coord, 3),
            self.adjacent_value_from_index(global_coord, 4),
            self.adjacent_value_from_index(global_coord, 5),
        ]
    }



    pub fn get_value(
        &mut self,
        coord: &IVec3,
    ) -> ValueType {
        if let Some(leaf_node) = self.eval_leaf_cache(coord) {
            return leaf_node.read().unwrap().get_value_at_coord(coord);
        } else if let Some(branch_node) = self.eval_branch_cache(coord) {
            let (value, leaf_node) = branch_node.read().unwrap().get_value_and_cache(coord);
            self.insert_leaf(coord, &leaf_node);
            return value;
        } else if let Some(trunk_node) = self.eval_trunk_cache(coord) {
            let (value, leaf_node, branch_node) = trunk_node.read().unwrap().get_value_and_cache(coord);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
            return value;
        } else {
            let (value, leaf_node, branch_node, trunk_node) = self.root_node.read().unwrap().get_value_and_cache(coord);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
            self.insert_trunk(coord, &trunk_node);
            return value;
        }
    }

    pub fn set_value(
        &mut self,
        coord: &IVec3,
        value: &ValueType,
        active: bool,
    ) {
        if let Some(leaf_node) = self.eval_leaf_cache(coord) {
            leaf_node.write().unwrap().set_value_at_coord(coord, value, active);
        } else if let Some(branch_node) = self.eval_branch_cache(coord) {
            let leaf_node = branch_node.write().unwrap().set_value_and_cache(coord, value, active);
            self.insert_leaf(coord, &leaf_node);
        } else if let Some(trunk_node) = self.eval_trunk_cache(coord) {
            let (leaf_node, branch_node) = trunk_node.write().unwrap().set_value_and_cache(coord, value, active);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
        } else {
            let (leaf_node, branch_node, trunk_node) = self.root_node.write().unwrap().set_value_and_cache(coord, value, active);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
            self.insert_trunk(coord, &trunk_node);
        }
    }

    pub fn set_value_on(
        &mut self,
        coord: &IVec3,
        value: &ValueType,
    ) {
        if let Some(leaf_node) = self.eval_leaf_cache(coord) {
            leaf_node.write().unwrap().set_value_at_coord(coord, value, true);
        } else if let Some(branch_node) = self.eval_branch_cache(coord) {
            let leaf_node = branch_node.write().unwrap().set_value_and_cache(coord, value, true);
            self.insert_leaf(coord, &leaf_node);
        } else if let Some(trunk_node) = self.eval_trunk_cache(coord) {
            let (leaf_node, branch_node) = trunk_node.write().unwrap().set_value_and_cache(coord, value, true);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
        } else {
            let (leaf_node, branch_node, trunk_node) = self.root_node.write().unwrap().set_value_and_cache(coord, value, true);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
            self.insert_trunk(coord, &trunk_node);
        }
    }

    /// If `ValueType` is [Voxel], use `set_voxel_off` instead
    pub fn set_value_off(
        &mut self,
        coord: &IVec3,
    ) {
        if let Some(leaf_node) = self.eval_leaf_cache(coord) {
            leaf_node.write().unwrap().set_value_at_coord(coord, &self.background, false);
        } else if let Some(branch_node) = self.eval_branch_cache(coord) {
            let leaf_node = branch_node.write().unwrap().set_value_and_cache(coord, &self.background, false);
            self.insert_leaf(coord, &leaf_node);
        } else if let Some(trunk_node) = self.eval_trunk_cache(coord) {
            let (leaf_node, branch_node) = trunk_node.write().unwrap().set_value_and_cache(coord, &self.background, false);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
        } else {
            let (leaf_node, branch_node, trunk_node) = self.root_node.write().unwrap().set_value_and_cache(coord, &self.background, false);
            self.insert_leaf(coord, &leaf_node);
            self.insert_branch(coord, &branch_node);
            self.insert_trunk(coord, &trunk_node);
        }
    }



    pub fn eval_trunk_cache(
        &mut self,
        coord: &IVec3,
    ) -> Option<Arc<RwLock<TrunkNode<ValueType>>>> {
        let coord_trunk_key = *coord & TRUNK_ORIGIN_MASK;
        if coord_trunk_key == self.trunk_key {
            self.trunk_node.clone()
        } else {
            self.trunk_key = IVec3::new(i32::MAX, i32::MAX, i32::MAX);
            None
        }
    }

    pub fn eval_branch_cache(
        &mut self,
        coord: &IVec3,
    ) -> Option<Arc<RwLock<BranchNode<ValueType>>>> {
        let coord_branch_key = *coord & BRANCH_ORIGIN_MASK;
        if coord_branch_key == self.branch_key {
            self.branch_node.clone()
        } else {
            self.branch_key = IVec3::new(i32::MAX, i32::MAX, i32::MAX);
            None
        }
    }

    pub fn eval_leaf_cache(
        &mut self,
        coord: &IVec3,
    ) -> Option<Arc<RwLock<LeafNode<ValueType>>>> {
        let coord_leaf_key = *coord & LEAF_ORIGIN_MASK;
        if coord_leaf_key == self.leaf_key {
            self.leaf_node.clone()
        } else {
            self.leaf_key = IVec3::new(i32::MAX, i32::MAX, i32::MAX);
            None
        }
    }



    pub fn insert_trunk(
        &mut self,
        coord: &IVec3,
        trunk: &Option<Arc<RwLock<TrunkNode<ValueType>>>>,
    ) {
        self.trunk_key = *coord & TRUNK_ORIGIN_MASK;
        self.trunk_node = trunk.clone();
    }

    pub fn insert_branch(
        &mut self,
        coord: &IVec3,
        branch: &Option<Arc<RwLock<BranchNode<ValueType>>>>,
    ) {
        self.branch_key = *coord & BRANCH_ORIGIN_MASK;
        self.branch_node = branch.clone();
    }

    pub fn insert_leaf(
        &mut self,
        coord: &IVec3,
        leaf: &Option<Arc<RwLock<LeafNode<ValueType>>>>,
    ) {
        self.leaf_key = *coord & LEAF_ORIGIN_MASK;
        self.leaf_node = leaf.clone();
    }
}