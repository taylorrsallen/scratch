//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use super::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
//================================-================================-================================ 
// TrunkNode
pub struct TrunkNode<ValueType: Clone + Copy + PartialEq + Eq> {
    origin: IVec3,
    nodes: Vec<TrunkData<ValueType>>,
    child_mask: Bitmask,
    value_mask: Bitmask,
    redraw_mask: Bitmask,
}

impl TrunkNode<Voxel> {
    pub fn get_meshes(
        &self,
        defs: &Res<Defs>,
        accessor: &mut Accessor<Voxel>,
    ) -> Vec<(IVec3, Option<Mesh>)>  {
        let mut key_mesh_pairs: Vec<(IVec3, Option<Mesh>)> = vec![];

        for index in OnMaskIter::new(0, &self.redraw_mask) {
            if self.child_mask.is_bit_on(index) {
                let branch_read_lock = self.nodes[index].child.as_ref().unwrap().read().unwrap();
                key_mesh_pairs.extend(branch_read_lock.get_meshes(defs, accessor));
            } else {
                error!("needed BRANCH tile mesh but developer is lazy!");
            }
        }

        key_mesh_pairs
    }
}

impl<ValueType: Clone + Copy + PartialEq + Eq> TrunkNode<ValueType> {
    pub fn new(
        coord: &IVec3,
        background: &ValueType,
        active: bool,
    ) -> Self {
        Self {
            origin: *coord & TRUNK_ORIGIN_MASK,
            nodes: vec![TrunkData::new_tile(background); TRUNK_SIZE],
            child_mask: Bitmask::from_cube_log2dim(TRUNK_LOG2DIM, false),
            value_mask: Bitmask::from_cube_log2dim(TRUNK_LOG2DIM, active),
            redraw_mask: Bitmask::from_cube_log2dim(TRUNK_LOG2DIM, active),
        }
    }

    pub fn index_from_coord(
        coord: &IVec3
    ) -> usize {
        ((((coord.x & (TRUNK_DIM as i32 - 1)) >> BRANCH_TOTAL) << TRUNK_LOG2DIM*2) +
         (((coord.y & (TRUNK_DIM as i32 - 1)) >> BRANCH_TOTAL) << TRUNK_LOG2DIM  ) +
         (((coord.z & (TRUNK_DIM as i32 - 1)) >> BRANCH_TOTAL))) as usize
    }



    pub fn assign_mesh_entity(
        &mut self,
        coord: &IVec3,
        new_mesh_entity: &Option<Entity>,
    ) -> Option<Entity> {
        let mut despawn_entity = None;

        let index = TrunkNode::<ValueType>::index_from_coord(coord);

        if self.child_mask.is_bit_on(index) {
            let mut branch_write_lock = self.nodes[index].child.as_ref().unwrap().write().unwrap();
            despawn_entity = branch_write_lock.assign_mesh_entity(coord, new_mesh_entity);
        } else {
            despawn_entity = self.nodes[index].mesh;
            self.nodes[index].mesh = *new_mesh_entity;
        }

        self.redraw_mask.set_bit_off(index);

        despawn_entity
    }

    pub fn set_redraw_and_cache(
        &mut self,
        coord: &IVec3,
    ) -> Option<Arc<RwLock<BranchNode<ValueType>>>> {
        let index = TrunkNode::<ValueType>::index_from_coord(coord);
        self.redraw_mask.set_bit_on(index);
        
        if self.child_mask.is_bit_on(index) {
            self.nodes[index].child.as_ref().unwrap().write().unwrap().set_redraw(coord);
            return Some(self.nodes[index].child.as_ref().unwrap().clone());
        }

        None
    }



    pub fn get_value_and_cache(
        &self,
        coord: &IVec3,
    ) -> (ValueType, Option<Arc<RwLock<LeafNode<ValueType>>>>, Option<Arc<RwLock<BranchNode<ValueType>>>>) {
        let index = TrunkNode::<ValueType>::index_from_coord(coord);
        if self.child_mask.is_bit_on(index) {
            let (value, leaf_node) = self.nodes[index].child.as_ref().unwrap().read().unwrap().get_value_and_cache(coord);
            let branch_node = Some(self.nodes[index].child.as_ref().unwrap().clone());
            return (value, leaf_node, branch_node);
        } else {
            return (self.nodes[index].tile, None, None);
        }
    }

    pub fn set_value_and_cache(
        &mut self,
        coord: &IVec3,
        value: &ValueType,
        active: bool,
    ) -> (Option<Arc<RwLock<LeafNode<ValueType>>>>, Option<Arc<RwLock<BranchNode<ValueType>>>>) {
        let index = TrunkNode::<ValueType>::index_from_coord(coord);
        let mut has_child = self.child_mask.is_bit_on(index);

        if !has_child {
            let background = self.nodes[index].tile;
            let active = self.value_mask.is_bit_on(index);
            if !active || background != *value {
                has_child = true;
                self.set_child_node(index, coord, &background, active);
            }
        }

        if has_child {
            let cached_leaf = self.nodes[index].child.as_ref().unwrap().write().unwrap().set_value_and_cache(coord, value, active);
            self.redraw_mask.set_bit_on(index);
            return (cached_leaf, Some(self.nodes[index].child.as_ref().unwrap().clone()));
        }

        (None, None)
    }



    pub fn set_child_node(
        &mut self,
        index: usize,
        coord: &IVec3,
        background: &ValueType,
        active: bool,
    ) {
        self.child_mask.set_bit_on(index);
        self.value_mask.set_bit_off(index);
        self.redraw_mask.set_bit_on(index);
        self.nodes[index].child = Some(Arc::new(RwLock::new(BranchNode::new(coord, &self.nodes[index].tile, active))));
    }
}

//================================-================================-================================ 
// BranchNode
pub struct BranchNode<ValueType: Clone + Copy + PartialEq + Eq> {
    origin: IVec3,
    nodes: Vec<BranchData<ValueType>>,
    child_mask: Bitmask,
    value_mask: Bitmask,
    redraw_mask: Bitmask,
}

impl BranchNode<Voxel> {
    pub fn get_meshes(
        &self,
        defs: &Res<Defs>,
        accessor: &mut Accessor<Voxel>,
    ) -> Vec<(IVec3, Option<Mesh>)> {
        let mut key_mesh_pairs: Vec<(IVec3, Option<Mesh>)> = vec![];

        for index in OnMaskIter::new(0, &self.redraw_mask) {
            if self.child_mask.is_bit_on(index) {
                let leaf_read_lock = self.nodes[index].child.as_ref().unwrap().read().unwrap();
                let key_mesh_pair = leaf_read_lock.get_mesh(defs, accessor);
                key_mesh_pairs.push(key_mesh_pair);
            } else {
                error!("needed LEAF tile mesh but developer is lazy!");
            }
        }

        key_mesh_pairs
    }
}

impl<ValueType: Clone + Copy + PartialEq + Eq> BranchNode<ValueType> {
    pub fn new(
        coord: &IVec3,
        background: &ValueType,
        active: bool,
    ) -> Self {
        Self {
            origin: *coord & BRANCH_ORIGIN_MASK,
            nodes: vec![BranchData::new_tile(background); BRANCH_SIZE],
            child_mask: Bitmask::from_cube_log2dim(BRANCH_LOG2DIM, false),
            value_mask: Bitmask::from_cube_log2dim(BRANCH_LOG2DIM, active),
            redraw_mask: Bitmask::from_cube_log2dim(BRANCH_LOG2DIM, active),
        }
    }

    pub fn index_from_coord(
        coord: &IVec3
    ) -> usize {
        ((((coord.x & (BRANCH_DIM as i32 - 1)) >> LEAF_TOTAL) << BRANCH_LOG2DIM*2) +
         (((coord.y & (BRANCH_DIM as i32 - 1)) >> LEAF_TOTAL) << BRANCH_LOG2DIM  ) +
         (((coord.z & (BRANCH_DIM as i32 - 1)) >> LEAF_TOTAL))) as usize
    }



    pub fn assign_mesh_entity(
        &mut self,
        coord: &IVec3,
        new_mesh_entity: &Option<Entity>,
    ) -> Option<Entity> {
        let index = BranchNode::<ValueType>::index_from_coord(coord);
        
        let despawn_entity = self.nodes[index].mesh;
        self.nodes[index].mesh = *new_mesh_entity;
        self.redraw_mask.set_bit_off(index);

        despawn_entity
    }

    pub fn set_redraw(
        &mut self,
        coord: &IVec3,
    ) {
        let index = BranchNode::<ValueType>::index_from_coord(coord);
        self.redraw_mask.set_bit_on(index);
    }



    pub fn get_value_and_cache(
        &self,
        coord: &IVec3,
    ) -> (ValueType, Option<Arc<RwLock<LeafNode<ValueType>>>>) {
        let index = BranchNode::<ValueType>::index_from_coord(coord);
        if self.child_mask.is_bit_on(index) {
            let value = self.nodes[index].child.as_ref().unwrap().read().unwrap().get_value_at_coord(coord);
            let leaf_node = Some(self.nodes[index].child.as_ref().unwrap().clone());
            return (value, leaf_node);
        } else {
            return (self.nodes[index].tile, None);
        }
    }

    pub fn set_value_and_cache(
        &mut self,
        coord: &IVec3,
        value: &ValueType,
        active: bool,
    ) -> Option<Arc<RwLock<LeafNode<ValueType>>>> {
        let index = BranchNode::<ValueType>::index_from_coord(coord);
        let mut has_child = self.child_mask.is_bit_on(index);

        if !has_child {
            let background = self.nodes[index].tile;
            let active = self.value_mask.is_bit_on(index);
            if !active || background != *value {
                has_child = true;
                self.set_child_node(index, coord, &background, active);
            }
        }

        if has_child {
            self.nodes[index].child.as_ref().unwrap().write().unwrap().set_value_at_coord(coord, value, active);
            self.redraw_mask.set_bit_on(index);
            return Some(self.nodes[index].child.as_ref().unwrap().clone());
        }

        None
    }



    pub fn set_child_node(
        &mut self,
        index: usize,
        coord: &IVec3,
        background: &ValueType,
        active: bool,
    ) {
        self.child_mask.set_bit_on(index);
        self.value_mask.set_bit_off(index);
        self.redraw_mask.set_bit_on(index);
        self.nodes[index].child = Some(Arc::new(RwLock::new(LeafNode::new(coord, &self.nodes[index].tile, active))));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
//================================-================================-================================ 
// TrunkData
#[derive(Clone)]
struct TrunkData<ValueType: Clone + Copy + PartialEq + Eq> {
    child: Option<Arc<RwLock<BranchNode<ValueType>>>>,
    tile: ValueType,

    mesh: Option<Entity>,
}

impl<ValueType: Clone + Copy + PartialEq + Eq> TrunkData<ValueType> {
    pub fn new_tile(
        background: &ValueType,
    ) -> Self {
        Self {
            child: None,
            tile: *background,

            mesh: None,
        }
    }
}

//================================-================================-================================ 
// BranchData
#[derive(Clone)]
struct BranchData<ValueType: Clone + Copy + PartialEq + Eq> {
    child: Option<Arc<RwLock<LeafNode<ValueType>>>>,
    tile: ValueType,

    mesh: Option<Entity>,
}

impl<ValueType: Clone + Copy + PartialEq + Eq> BranchData<ValueType> {
    pub fn new_tile(
        background: &ValueType,
    ) -> Self {
        Self {
            child: None,
            tile: *background,

            mesh: None,
        }
    }
}