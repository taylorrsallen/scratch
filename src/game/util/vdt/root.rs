//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use super::*;
use bevy::utils::HashMap;
pub use std::sync::{
    Arc,
    RwLock,
};

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct RootNode<ValueType: Clone + Copy + PartialEq + Eq> {
    table: HashMap<IVec3, RootData<ValueType>>,
    pub background: ValueType,
}

impl RootNode<Voxel> {
    pub fn get_meshes(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        voxels: &mut Accessor<Voxel>,
        voxel_materials: &VoxelMaterials,
        defs: &Res<Defs>,
    ) -> Vec<(IVec3, Option<Entity>)> {
        let mut key_mesh_entity_pairs: Vec<(IVec3, Option<Entity>)> = vec![];

        for root_data in self.table.values() {
            if root_data.redraw {
                if let Some(trunk_node) = &root_data.child {
                    let trunk_read_lock = trunk_node.read().unwrap();
                    let key_mesh_pairs = trunk_read_lock.get_meshes(defs, voxels);

                    for (key, mesh) in key_mesh_pairs.iter() {
                        if let Some(mesh) = mesh {
                            let mesh_entity = commands.spawn(PbrBundle {
                                    mesh: meshes.add(mesh.clone()),
                                    material: voxel_materials.base_material.clone(),
                                    transform: Transform::from_translation(key.as_vec3()),
                                    global_transform: GlobalTransform::from(Transform::from_translation(key.as_vec3())),
                                    ..default()
                                })
                                .insert(RigidBody::Fixed)
                                .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
                                .insert(Name::new("Mesh ".to_string() + &key.to_string()))
                                .id();
    
                            key_mesh_entity_pairs.push((*key, Some(mesh_entity)));
                        } else {
                            key_mesh_entity_pairs.push((*key, None));
                        }
                    }
                } else {
                    error!("needed TRUNK tile mesh but developer is lazy!");
                }
            }
        }

        key_mesh_entity_pairs
    }

    pub fn assign_meshes(
        &mut self,
        coord_mesh_entity_pairs: &Vec<(IVec3, Option<Entity>)>,
        commands: &mut Commands,
    ) {
        let mut despawn: Vec<Entity> = vec![];

        for (coord, mesh_entity) in coord_mesh_entity_pairs.iter() {
            despawn.extend(self.assign_mesh_entity(coord, &mesh_entity));
        }

        for entity in despawn.iter() {
            commands.entity(*entity).despawn_recursive();
        }
    }

    fn assign_mesh_entity(
        &mut self,
        coord: &IVec3,
        new_mesh_entity: &Option<Entity>,
    ) -> Vec<Entity> {
        let mut despawn: Vec<Entity> = vec![];

        let key = *coord & TRUNK_ORIGIN_MASK;
        if let Some(root_data) = self.table.get_mut(&key) {
            root_data.redraw = false;
            if let Some(trunk_node) = &root_data.child {
                let mut trunk_write_lock = trunk_node.write().unwrap();
                despawn.extend(trunk_write_lock.assign_mesh_entity(coord, new_mesh_entity));
            } else {
                if let Some(old_mesh_entity) = root_data.mesh {
                    despawn.push(old_mesh_entity);
                }

                root_data.mesh = *new_mesh_entity;
            }
        } else {
            error!("assign_mesh_entity failed to find struct for mesh - this should never happen");
        }

        despawn
    }

    pub fn set_redraw_and_cache(
        &mut self,
        coord: &IVec3,
    ) -> (Option<Arc<RwLock<BranchNode<Voxel>>>>, Option<Arc<RwLock<TrunkNode<Voxel>>>>) {
        let key = *coord & TRUNK_ORIGIN_MASK;
        let mut child: Option<Arc<RwLock<TrunkNode<Voxel>>>> = None;
        if let Some(root_data) = self.table.get_mut(&key) {

            root_data.redraw = true;

            if let Some(trunk_node) = &root_data.child {
                child = Some(trunk_node.clone());
            }
        } else {
            self.table.insert(key, RootData::new_tile(&self.background));
        }

        if let Some(child) = child {
            let branch_node = child.write().unwrap().set_redraw_and_cache(coord);
            return (branch_node, Some(child));
        }

        (None, None)
    }
}

impl<ValueType: Clone + Copy + PartialEq + Eq> RootNode<ValueType> {
    pub fn new(
        background: ValueType,
    ) -> Self {
        Self {
            table: HashMap::<IVec3, RootData<ValueType>>::default(),
            background,
        }
    }



    pub fn get_value_and_cache(
        &self,
        coord: &IVec3,
    ) -> (ValueType, Option<Arc<RwLock<LeafNode<ValueType>>>>, Option<Arc<RwLock<BranchNode<ValueType>>>>, Option<Arc<RwLock<TrunkNode<ValueType>>>>) {
        let key = *coord & TRUNK_ORIGIN_MASK;
        if let Some(root_data) = self.table.get(&key) {
            if let Some(trunk_node) = &root_data.child {
                let (value, leaf_node, branch_node) = trunk_node.read().unwrap().get_value_and_cache(coord);
                return (value, leaf_node, branch_node, Some(trunk_node.clone()));
            } else {
                return (root_data.tile, None, None, None);
            }
        } else {
            return (self.background, None, None, None);
        }
    }

    pub fn set_value_and_cache(
        &mut self,
        coord: &IVec3,
        value: &ValueType,
        active: bool,
    ) -> (Option<Arc<RwLock<LeafNode<ValueType>>>>, Option<Arc<RwLock<BranchNode<ValueType>>>>, Option<Arc<RwLock<TrunkNode<ValueType>>>>) {
        let key = *coord & TRUNK_ORIGIN_MASK;
        let mut child: Option<Arc<RwLock<TrunkNode<ValueType>>>> = None;
        if let Some(root_data) = self.table.get_mut(&key) {

            root_data.redraw = true;

            if let Some(trunk_node) = &root_data.child {
                child = Some(trunk_node.clone());
            } else if root_data.tile != *value {
                let new_child = Arc::new(RwLock::new(TrunkNode::new(coord, &root_data.tile, root_data.tile != self.background)));
                root_data.child = Some(new_child.clone());
                child = Some(new_child);
            }
        } else {
            let new_child = Arc::new(RwLock::new(TrunkNode::new(coord, &self.background, false)));
            self.table.insert(key, RootData::new_child(&new_child, &self.background));
            child = Some(new_child);
        }

        if let Some(child) = child {
            let (leaf_node, branch_node) = child.write().unwrap().set_value_and_cache(coord, value, active);
            return (leaf_node, branch_node, Some(child));
        }

        (None, None, None)
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
struct RootData<ValueType: Clone + Copy + PartialEq + Eq> {
    child: Option<Arc<RwLock<TrunkNode<ValueType>>>>,
    tile: ValueType,

    mesh: Option<Entity>,
    redraw: bool,
}

impl<ValueType: Clone + Copy + PartialEq + Eq> RootData<ValueType> {
    pub fn new_tile(
        background: &ValueType,
    ) -> Self {
        Self {
            child: None,
            tile: *background,

            mesh: None,
            redraw: true,
        }
    }

    pub fn new_child(
        child: &Arc<RwLock<TrunkNode<ValueType>>>,
        background: &ValueType,
    ) -> Self {
        Self {
            child: Some(child.clone()),
            tile: *background,

            mesh: None,
            redraw: true,
        }
    }
}