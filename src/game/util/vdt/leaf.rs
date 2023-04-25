//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use super::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct LeafNode<ValueType> {
    origin: IVec3,
    data: [ValueType; LEAF_SIZE],
    value_mask: Bitmask,
}

impl LeafNode<Voxel> {
    pub fn get_mesh(
        &self,
        defs: &Res<Defs>,
        accessor: &mut Accessor<Voxel>,
    ) -> (IVec3, Option<Mesh>) {
        let mut mesh_data = MeshData::default();

        for index in OnMaskIter::new(0, &self.value_mask) {
            mesh_data.add_cube_voxel(&self.origin, index, &self.data[index], defs, accessor);
        }

        if mesh_data.is_empty() {
            (self.origin, None)
        } else {
            (self.origin, Some(mesh_data.get_mesh()))
        }
    }
}

impl<ValueType: Clone + Copy + PartialEq + Eq> LeafNode<ValueType> {
    pub fn new(
        coord: &IVec3,
        background: &ValueType,
        active: bool,
    ) -> Self {
        Self {
            origin: *coord & LEAF_ORIGIN_MASK,
            data: [*background; LEAF_SIZE],
            value_mask: Bitmask::from_cube_log2dim(LEAF_LOG2DIM, active),
        }
    }



    pub fn index_from_coord(
        coord: &IVec3
    ) -> usize {
        (((coord.x & (LEAF_DIM as i32 - 1)) << LEAF_LOG2DIM*2) +
         ((coord.y & (LEAF_DIM as i32 - 1)) << LEAF_LOG2DIM  ) +
          (coord.z & (LEAF_DIM as i32 - 1))) as usize
    }

    pub fn local_coord_from_index(
        index: usize,
    ) -> IVec3 {
        let x = (index >> (LEAF_LOG2DIM*2)) as i32;
        let n = index & ((1 << LEAF_LOG2DIM*2) - 1);
        let y = (n >> LEAF_LOG2DIM) as i32;
        let z = (n & ((1 << LEAF_LOG2DIM) - 1)) as i32;
        
        IVec3::new(x, y, z)
    }

    pub fn global_coord_from_index(
        &self,
        index: usize,
    ) -> IVec3 {
        LeafNode::<ValueType>::local_coord_from_index(index) + self.origin
    }



    pub fn get_value_at_index(
        &self,
        index: usize,
    ) -> ValueType {
        self.data[index]
    }

    pub fn get_value_at_coord(
        &self,
        coord: &IVec3,
    ) -> ValueType {
        self.get_value_at_index(LeafNode::<ValueType>::index_from_coord(coord))
    }



    pub fn set_value_at_index(
        &mut self,
        index: usize,
        value: &ValueType,
        active: bool,
    ) {
        self.data[index] = *value;
        self.value_mask.set_bit(index, active);
    }

    pub fn set_value_at_coord(
        &mut self,
        coord: &IVec3,
        value: &ValueType,
        active: bool,
    ) {
        self.set_value_at_index(LeafNode::<ValueType>::index_from_coord(coord), value, active);
    }
}