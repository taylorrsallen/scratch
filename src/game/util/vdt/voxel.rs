//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use super::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
pub const GRID_DIRECTIONS: [IVec3; 6] = [
    IVec3::new(-1,  0,  0), // Left
    IVec3::new( 1,  0,  0), // Right
    IVec3::new( 0, -1,  0), // Bottom
    IVec3::new( 0,  1,  0), // Top
    IVec3::new( 0,  0, -1), // Back
    IVec3::new( 0,  0,  1), // Front
];

pub const VOXEL_SIDE_FACE_CHECKS: [IVec3; 4] = [
    IVec3::new(-1,  0,  0), // Left
    IVec3::new( 1,  0,  0), // Right
    IVec3::new( 0,  0, -1), // Back
    IVec3::new( 0,  0,  1), // Front
];

pub const VOXEL_VERTICAL_FACE_CHECKS: [IVec3; 2] = [
    IVec3::new( 0, -1,  0), // Bottom
    IVec3::new( 0,  1,  0), // Top
];

pub const VOXEL_FACE_CHECKS_WITH_VERTICAL_DIAG: [IVec3; 14] = [
    IVec3::new(-1,  0,  0), // Left
    IVec3::new( 1,  0,  0), // Right
    IVec3::new( 0, -1,  0), // Bottom
    IVec3::new( 0,  1,  0), // Top
    IVec3::new( 0,  0, -1), // Back
    IVec3::new( 0,  0,  1), // Front
    IVec3::new(-1, -1,  0), // Left-Bottom
    IVec3::new(-1,  1,  0), // Left-Top
    IVec3::new( 1, -1,  0), // Right-Bottom
    IVec3::new( 1,  1,  0), // Right-Top
    IVec3::new( 0, -1, -1), // Back-Bottom
    IVec3::new( 0,  1, -1), // Back-Top
    IVec3::new( 0, -1,  1), // Front-Bottom
    IVec3::new( 0,  1,  1), // Front-Top
];

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridDirection {
    Left,
    Right,
    Bottom,
    Top,
    Back,
    Front,
}

impl GridDirection {
    pub fn as_ivec3(&self) -> IVec3 {
        GRID_DIRECTIONS[*self as usize]
    }
}

// First 2 bits of Voxel.matter
// 00 = Empty
// 10 = Gas
// 01 = Liquid
// 11 = Solid
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VoxelMatterState {
    Void   = 0x0,
    Gas    = 0x1,
    Liquid = 0x2, // ON [Liquid]: 
    Solid  = 0x3,
}

// Last 6 bits of Voxel.matter
// Liquid: Soft + Loose + Light = Water
// Liquid: Soft + Loose + Heavy + Opaque = Lava

// Solid: Soft + Loose + Light + Opaque = Sand
// Solid: Hard + Loose + Light + Opaque = Gravel
// Solid: Soft + Dense + Light + Opaque = Dirt
// Solid: Hard + Dense + Heavy + Opaque = Stone
// Solid: Hard + Loose + Light = Ice
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VoxelMatterProperty {
    // ON [Hard]:
        // AS SOLID:
            // connects with hard voxels
            // resists bending, prefer breaking
    // OFF [Soft]:
        // AS SOLID:
            // never connects with voxels
            // resists breaking, prefer bending
    Hard = 0x04,

    // ON [Dense]:
        // resistant
        // slow as liquid
    // OFF [Loose]: 
        // falls apart
        // fast as liquid
    Dense = 0x08,

    // ON [Heavy]:
        // faster gravity acceleration
        // more strength to move
    // OFF [Light]:
        // slower gravity acceleration
        // less strength to move
    Heavy = 0x10,

    // Self explanatory
    Opaque = 0x20,

    Undefined01 = 0x40,
    Undefined02 = 0x80,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VoxelState {
    None = 0x00,
    Blocked = 0x80,
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Voxel {
    matter_id: u8,
    state: u8,
}

impl Default for Voxel {
    fn default() -> Self {
        Self { matter_id: 0, state: 0 }
    }
}

impl Voxel {
    pub fn new(matter_id: u8, state: u8) -> Self {
        Self { matter_id, state }
    }

    pub fn from_matter_id(matter_id: u8) -> Self {
        Self { matter_id, state: 0 }
    }

    pub fn face_texture_id(&self, face: u8, defs: &Res<Defs>) -> u32 {
        defs.matter.get_u8(self.matter_id).texture_id(face)
    }

    /// Only solids can be opaque, but not all solids are opaque
    pub fn is_opaque(&self, defs: &Res<Defs>) -> bool {
        defs.matter.get_u8(self.matter_id).is_opaque()
    }

    pub fn is_solid(&self, defs: &Res<Defs>) -> bool {
        defs.matter.get_u8(self.matter_id).is_solid()
    }

    pub fn is_blocked(&self, defs: &Res<Defs>) -> bool {
        defs.matter.get_u8(self.matter_id).is_solid() || self.state & VoxelState::Blocked as u8 == VoxelState::Blocked as u8
    }

    /// True if this voxel is not blocked and the voxel beneath it is
    pub fn is_walkable(&self, coord: &IVec3, voxels: &mut Accessor<Voxel>, defs: &Res<Defs>) -> bool {
        !self.is_blocked(defs) && voxels.adjacent_value_from_direction(coord, GridDirection::Bottom).is_blocked(defs)
    }
}