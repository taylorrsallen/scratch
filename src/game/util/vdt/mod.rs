//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

mod root;
pub use root::*;
mod internal;
pub use internal::*;
mod leaf;
pub use leaf::*;

mod voxel;
pub use voxel::*;

mod accessor;
pub use accessor::*;
mod mesh;
pub use mesh::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
const LEAF_LOG2DIM: usize = 3;
const BRANCH_LOG2DIM: usize = 4;
const TRUNK_LOG2DIM: usize = 5;

const LEAF_TOTAL: usize =     LEAF_LOG2DIM;
const LEAF_DIM: usize =       1 << LEAF_TOTAL;
const LEAF_SIZE: usize =      1 << (LEAF_LOG2DIM*3);
const LEAF_MASK_SIZE: usize = LEAF_SIZE >> 6;
const LEAF_ORIGIN_MASK: i32 = !(LEAF_DIM as i32 - 1);

const BRANCH_TOTAL: usize =     BRANCH_LOG2DIM + LEAF_TOTAL;
const BRANCH_DIM: usize =       1 << BRANCH_TOTAL;
const BRANCH_SIZE: usize =      1 << (BRANCH_LOG2DIM*3);
const BRANCH_MASK_SIZE: usize = BRANCH_SIZE >> 6;
const BRANCH_ORIGIN_MASK: i32 = !(BRANCH_DIM as i32 - 1);

const TRUNK_TOTAL: usize =     TRUNK_LOG2DIM + BRANCH_TOTAL;
const TRUNK_DIM: usize =       1 << TRUNK_TOTAL;
const TRUNK_SIZE: usize =      1 << (TRUNK_LOG2DIM*3);
const TRUNK_MASK_SIZE: usize = TRUNK_SIZE >> 6;
const TRUNK_ORIGIN_MASK: i32 = !(TRUNK_DIM as i32 - 1);

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct VdtPlugin;
impl Plugin for VdtPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_system(sys_remesh_voxel_trees.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(sys_remesh_voxel_trees); // .run_if(on_timer(Duration::from_secs_f32(0.1))));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
//================================-================================-================================ 
// VoxelTreeBundle
#[derive(Bundle)]
pub struct VoxelTreeBundle {
    tree: VoxelTree,
    materials: VoxelMaterials,
    transform: Transform,
}

impl VoxelTreeBundle {
    pub fn new(
        translation: &Vec3,
        background: Voxel,
        materials: VoxelMaterials,
    ) -> Self {
        let mut root_node = Arc::new(RwLock::new(RootNode::new(background)));
        Self {
            tree: VoxelTree::from_root_node(&root_node),
            materials,
            transform: Transform::from_translation(*translation),
        }
    }

    pub fn from_tree(
        translation: &Vec3,
        tree: VoxelTree,
        materials: VoxelMaterials,
    ) -> Self {
        Self {
            tree,
            materials,
            transform: Transform::from_translation(*translation),
        }
    }
}

//================================-================================-================================ 
// LevelTreeBundle
#[derive(Bundle)]
pub struct LevelTreeBundle {
    name: Name,
    voxel_tree: VoxelTree,
    entity_tree: EntityTree,
    level_tree: LevelTree,
    materials: VoxelMaterials,
    transform: Transform,
}

impl LevelTreeBundle {
    pub fn new(
        background: Voxel,
        materials: VoxelMaterials,
    ) -> Self {
        let voxel_root = Arc::new(RwLock::new(RootNode::new(background)));
        Self {
            name: Name::new("Level Tree"),
            voxel_tree: VoxelTree::from_root_node(&voxel_root),
            entity_tree: EntityTree::new(),
            level_tree: LevelTree::default(),
            materials,
            transform: Transform::IDENTITY,
        }
    }

    pub fn from_voxel_tree(
        tree: VoxelTree,
        materials: VoxelMaterials,
    ) -> Self {
        Self {
            name: Name::new("Level Tree"),
            voxel_tree: tree,
            entity_tree: EntityTree::new(),
            level_tree: LevelTree::default(),
            materials,
            transform: Transform::IDENTITY,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
//================================-================================-================================ 
// LevelTree
#[derive(Default, Component)]
pub struct LevelTree;

//================================-================================-================================ 
// VoxelTree
#[derive(Component)]
pub struct VoxelTree {
    root: Arc<RwLock<RootNode<Voxel>>>,
}

impl VoxelTree {
    pub fn new(
        background: Voxel,
    ) -> Self {
        Self {
            root: Arc::new(RwLock::new(RootNode::new(background))),
        }
    }

    pub fn from_root_node(
        root_node: &Arc<RwLock<RootNode<Voxel>>>,
    ) -> Self {
        Self {
            root: root_node.clone(),
        }
    }
    
    pub fn get_accessor(
        &self,
    ) -> Accessor<Voxel> {
        Accessor::new(&self.root)
    }
}

//================================-================================-================================ 
// EntityTree
#[derive(Component)]
pub struct EntityTree {
    root: Arc<RwLock<RootNode<Option<Entity>>>>,
}

impl EntityTree {
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(RootNode::new(None))),
        }
    }

    pub fn from_root_node(
        root: &Arc<RwLock<RootNode<Option<Entity>>>>,
    ) -> Self {
        Self {
            root: root.clone(),
        }
    }
    
    pub fn get_accessor(
        &self,
    ) -> Accessor<Option<Entity>> {
        Accessor::new(&self.root)
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_remesh_voxel_trees(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tree_query: Query<(&mut VoxelTree, &VoxelMaterials)>,
    defs: Res<Defs>,
) {
    for (tree, materials) in tree_query.iter_mut() {
        let mut voxels = Accessor::new(&tree.root);
        let key_mesh_entity_pairs = tree.root.read().unwrap().get_meshes(&mut commands, &mut meshes, &mut voxels, &materials, &defs);
        tree.root.write().unwrap().assign_meshes(&key_mesh_entity_pairs, &mut commands);
    }
}