//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use super::*;
use bevy::render::{
    render_resource::PrimitiveTopology,
    mesh::Indices
};

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
pub const TEX_ATLAS_DIM: usize = 32;
pub const TEX_ATLAS_UV_DIM: f32 = 1.0 / TEX_ATLAS_DIM as f32;
pub const CUBE_HALF_DIM: f32 = 0.5;

pub const CUBE_VERTS: [[f32; 3]; 8] = [
    [-CUBE_HALF_DIM, -CUBE_HALF_DIM, -CUBE_HALF_DIM], // 0 Left,  Bottom, Back
    [ CUBE_HALF_DIM, -CUBE_HALF_DIM, -CUBE_HALF_DIM], // 1 Right, Bottom, Back
    [-CUBE_HALF_DIM,  CUBE_HALF_DIM, -CUBE_HALF_DIM], // 2 Left,  Top,    Back,
    [ CUBE_HALF_DIM,  CUBE_HALF_DIM, -CUBE_HALF_DIM], // 3 Right, Top,    Back,
    [-CUBE_HALF_DIM, -CUBE_HALF_DIM,  CUBE_HALF_DIM], // 4 Left,  Bottom, Front,
    [ CUBE_HALF_DIM, -CUBE_HALF_DIM,  CUBE_HALF_DIM], // 5 Right, Bottom, Front,
    [-CUBE_HALF_DIM,  CUBE_HALF_DIM,  CUBE_HALF_DIM], // 6 Left,  Top,    Front,
    [ CUBE_HALF_DIM,  CUBE_HALF_DIM,  CUBE_HALF_DIM], // 7 Right, Top,    Front,
];

pub const CUBE_NORMALS: [[f32; 3]; 6] = [
    [-1.0,  0.0,  0.0], // Left face
    [ 1.0,  0.0,  0.0], // Right face
    [ 0.0, -1.0,  0.0], // Bottom face
    [ 0.0,  1.0,  0.0], // Top face
    [ 0.0,  0.0, -1.0], // Back face
    [ 0.0,  0.0,  1.0], // Front face
];

pub const CUBE_UVS: [[f32; 2]; 4] = [
    [TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.01, TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.01],
    [TEX_ATLAS_UV_DIM * 0.01                   , TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.01],
    [TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.01, TEX_ATLAS_UV_DIM * 0.01                   ],
    [TEX_ATLAS_UV_DIM * 0.01                   , TEX_ATLAS_UV_DIM * 0.01                   ],
];

pub const CUBE_QUAD_VERTS: [[usize; 4]; 6] = [ // Read as: 0, 1, 2, 2, 1, 3
    [4, 0, 6, 2], // Left face
    [1, 5, 3, 7], // Right face
    [4, 5, 0, 1], // Bottom face
    [2, 3, 6, 7], // Top face
    [0, 1, 2, 3], // Back face
    [5, 4, 7, 6], // Front face
];

pub const CUBE_QUAD_INDICES: [u32; 6] = [0, 2, 1, 1, 2, 3];

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct MeshData {
    verts: Vec<[f32; 3]>,
    indices: Vec<u32>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
}

impl Default for MeshData {
    fn default() -> Self {
        Self {
            verts: vec![],
            indices: vec![],
            uvs: vec![],
            normals: vec![],
        }
    }
}

impl MeshData {
    pub fn is_empty(
        &self,
    ) -> bool {
        self.verts.is_empty()
    }

    pub fn get_mesh(
        mut self,
    ) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.verts);
        mesh.set_indices(Some(Indices::U32(self.indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        mesh
    }

    pub fn add_cube_voxel(
        &mut self,
        leaf_origin: &IVec3,
        index: usize,
        voxel: &Voxel,
        defs: &Res<Defs>,
        accessor: &mut Accessor<Voxel>,
    ) {
        let local_coord = LeafNode::<Voxel>::local_coord_from_index(index);
        let global_coord = local_coord + *leaf_origin;

        let neighbors = accessor.get_adjacent_values(&global_coord);
    
        for face in 0..6 {
            if neighbors[face as usize].is_opaque(defs) {
                continue;
            }

            let vert_count = self.verts.len();
    
            for vert_index in 0..4 {
                let vert = CUBE_VERTS[CUBE_QUAD_VERTS[face as usize][vert_index]];
                self.verts.push([
                    vert[0] + local_coord.x as f32,
                    vert[1] + local_coord.y as f32,
                    vert[2] + local_coord.z as f32,
                ]);
    
                let uv = CUBE_UVS[vert_index];
                let uv_offset = TEX_ATLAS_UV_DIM * voxel.face_texture_id(face, defs) as f32;
                let uv_offset_floor = uv_offset.floor() as f32;
                self.uvs.push([
                    uv[0] + uv_offset - uv_offset_floor,
                    uv[1] + uv_offset_floor * TEX_ATLAS_UV_DIM as f32,
                ]);
            }
    
            self.normals.extend(vec![CUBE_NORMALS[face as usize]; 4]);
    
            for tri_index in 0..6 {
                self.indices.push(CUBE_QUAD_INDICES[tri_index] + vert_count as u32);
            }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct VoxelMaterials {
    pub base_material: Handle<StandardMaterial>,
}

impl VoxelMaterials {
    pub fn new_world_materials(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_loader: &Res<AssetLoader>,
    ) -> Self {
        Self {
            base_material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(asset_loader.images.get_handle("tex_atlas")),
                    perceptual_roughness: 0.9,
                    reflectance: 0.0,
                    ..default()
                }),
        }
    }
}