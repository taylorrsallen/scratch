//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct DefsPlugin;
impl Plugin for DefsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Defs::from_load_or_default());
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource)]
pub struct Defs {
    /// Used for voxels, things that items can be made of, the types of matter that exist in a world
    pub matter: DefType<MatterDef>,
    /// Voxel and entity data to save/load(?)
    pub levels: DefType<LevelDef>,
    pub items: DefType<ItemDef>,
    pub actors: DefType<ActorDef>,
    pub abilities: DefType<AbilityDef>,
    pub projectiles: DefType<ProjectileDef>,
}

impl Default for Defs {
    fn default() -> Self {
        Self {
            matter: DefType::<MatterDef>::new("matter"),
            levels: DefType::<LevelDef>::new("levels"),
            items: DefType::<ItemDef>::new("items"),
            actors: DefType::<ActorDef>::new("actors"),
            abilities: DefType::<AbilityDef>::new("abilities"),
            projectiles: DefType::<ProjectileDef>::new("projectiles"),
        }
    }
}

impl Defs {
    pub fn from_load_or_default() -> Self {
        let mut defs = Self::default();

        defs.matter.init_load_or_default();
        defs.levels.init_load_or_default();
        defs.items.init_load_or_default();
        defs.actors.init_load_or_default();
        defs.abilities.init_load_or_default();
        defs.projectiles.init_load_or_default();

        for matter in defs.matter.defs.iter_mut() {
            matter.init();
        }

        defs
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct DefType<T: Default + Serialize + for<'a> Deserialize<'a>> {
    name: String,
    defs: Vec<T>,
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> Default for DefType<T> {
    fn default() -> Self {
        Self {
            name: "NULL".into(),
            defs: vec![],
        }
    }
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> DefType<T> {
    fn new(
        name: &str,
    ) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }

    fn init_load_or_default(
        &mut self,
    ) {
        if let Some(contents) = Data::try_read_file_to_string(&("assets/defs/".to_string() + &self.name + ".ron")) {
            if let Ok(defs) = ron::from_str::<Vec<T>>(&contents) {
                self.defs = defs;
                return;
            } else {
                panic!("{} defs present but failed to load!", self.name);
            }
        }
        
        self.defs.push(T::default());
        if let Ok(contents) = Data::to_ron_string_pretty(&self.defs) {
            Data::try_write_file(&("assets/defs/".to_string() + &self.name + ".ron"), contents.as_bytes());
        }
    }

    pub fn get(
        &self,
        id: u32,
    ) -> &T {
        if id as usize >= self.defs.len() {
            &self.defs[0]
        } else {
            &self.defs[id as usize]
        }
    }

    pub fn get_u8(
        &self,
        id: u8,
    ) -> &T {
        if id as usize >= self.defs.len() {
            &self.defs[0]
        } else {
            &self.defs[id as usize]
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// DEFS
//================================-================================-================================ 
// Matter
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatterDef {
    name: String,
    state: Option<VoxelMatterState>,
    properties: Option<Vec<VoxelMatterProperty>>,
    texture_id_data: Option<[u32; 6]>,
    meta_texture_id_data: Option<[u32; 15]>,
    
    #[serde(skip)]
    matter_mask: u16,
    #[serde(skip)]
    texture_ids: [u32; 6],
    #[serde(skip)]
    meta_texture_ids: [u32; 15],
}

impl Default for MatterDef {
    fn default() -> Self {
        Self {
            name: "void".into(),
            state: None,
            properties: None,
            texture_id_data: None,
            meta_texture_id_data: None,
            
            matter_mask: 0,
            texture_ids: [0; 6],
            meta_texture_ids: [0; 15],
        }
    }
}

impl MatterDef {
    pub fn init(
        &mut self,
    ) {
        if let Some(state) = self.state {
            self.matter_mask |= state as u16;
        }

        if let Some(properties) = &self.properties {
            for matter_property in properties.iter() {
                self.matter_mask |= *matter_property as u16;
            }
        }

        if let Some(texture_id_data) = self.texture_id_data {
            self.texture_ids = texture_id_data;
        }

        if let Some(meta_texture_id_data) = self.meta_texture_id_data {
            self.meta_texture_ids = meta_texture_id_data;
        }
    }

    pub fn texture_id(
        &self,
        face: u8,
    ) -> u32 {
        self.texture_ids[face as usize]
    }

    pub fn meta_texture_id(
        &self,
        index: u8,
    ) -> u32 {
        self.meta_texture_ids[index as usize]
    }

    pub fn is_opaque(
        &self,
    ) -> bool {
        self.matter_mask & (VoxelMatterState::Solid as u16 | VoxelMatterProperty::Opaque as u16) == (VoxelMatterState::Solid as u16 | VoxelMatterProperty::Opaque as u16)
    }

    pub fn is_solid(
        &self,
    ) -> bool {
        self.matter_mask & VoxelMatterState::Solid as u16 == VoxelMatterState::Solid as u16
    }
}

//================================-================================-================================ 
// Level
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelDef {
    name: String,
    
}

impl Default for LevelDef {
    fn default() -> Self {
        Self {
            name: "dungeon_entrance".into(),
        }
    }
}

//================================-================================-================================ 
// Item
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemDef {
    name: String,
    
}

impl Default for ItemDef {
    fn default() -> Self {
        Self {
            name: "sword".into(),
        }
    }
}

//================================-================================-================================ 
// Actor
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorDef {
    name: String,
    mesh: String,
}

impl Default for ActorDef {
    fn default() -> Self {
        Self {
            name: "demon".into(),
            mesh: "cube".into(),
        }
    }
}

//================================-================================-================================ 
// Ability
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityResource {
    Health,
    Stamina,
    Mana,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityResourceDrain {
    Instant(u32),
    Continuous { drain_per_step: u32 },
    Timed { steps: u32, drain_per_step: u32 },
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityCost(AbilityResource, AbilityResourceDrain);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityDef {
    pub name: String,
    costs: Vec<AbilityCost>,
    projectiles: Vec<u32>,
    valid_targets: Vec<SelectionType>,

    #[serde(skip)]
    valid_targets_mask: u8,
}

impl Default for AbilityDef {
    fn default() -> Self {
        Self {
            name: "Throw Hammer".into(),
            costs: vec![AbilityCost(AbilityResource::Stamina, AbilityResourceDrain::Instant(5))],
            projectiles: vec![0],
            valid_targets: vec![SelectionType::Coord, SelectionType::Unit],
            valid_targets_mask: SelectionType::Coord as u8 | SelectionType::Unit as u8,
        }
    }
}

impl AbilityDef {
    pub fn is_selection_valid_target(&self, selection: &Selection) -> bool {
        let mut mask = 0;
        match *selection {
            Selection::Coord(_)  => { mask |= SelectionType::Coord as u8 }
            Selection::Coords(_) => { mask |= SelectionType::Coords as u8 }
            Selection::Area(_)   => { mask |= SelectionType::Area as u8 }
            Selection::Areas(_)  => { mask |= SelectionType::Areas as u8 }
            Selection::Unit(_)   => { mask |= SelectionType::Unit as u8 }
            Selection::Units(_)  => { mask |= SelectionType::Units as u8 }
        }

        mask & self.valid_targets_mask == mask
    }
}

//================================-================================-================================ 
// Projectile
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectileDef {
    pub name: String,
    mesh: String,
    scale: f32,
    steps_per_voxel: u32,
}

impl Default for ProjectileDef {
    fn default() -> Self {
        Self {
            name: "Hammer".into(),
            mesh: "hammer".into(),
            scale: 1.0,
            steps_per_voxel: 5,
        }
    }
}

impl ProjectileDef {

}