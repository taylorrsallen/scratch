//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use std::fs::*;
use bevy::{
    utils::HashMap,
    asset::{Asset, LoadState},
    render::{
        texture::ImageSampler,
        render_resource::{
            SamplerDescriptor,
            AddressMode,
            FilterMode
        },
    },
};

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((
                    stsys_init_asset_loader,
                ).in_base_set(StartupSet::PreStartup))
            .add_systems((
                    sys_set_image_samplers,
                ));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
#[derive(Default)]
pub struct AssetType<T: Asset> {
    pub name: String,
    pub extension: String,
    pub ids: HashMap<String, usize>,
    pub assets: Vec<Handle<T>>,
}

impl<T: Asset> AssetType<T> {
    pub fn new(
        name: &str,
        extension: &str,
    ) -> Self {
        Self {
            name: name.into(),
            extension: extension.into(),
            ids: HashMap::<String, usize>::default(),
            assets: vec![],
        }
    }



    pub fn get_handle(
        &self,
        asset: &str,
    ) -> Handle<T> {
        if let Some(asset_index) = self.ids.get(&(self.name.clone() + "/" + asset)) {
            return self.assets[*asset_index].clone();
        } else {
            return self.assets[0].clone();
        }
    }



    pub fn load_assets(
        &mut self,
        asset_server: &Res<AssetServer>,
    ) {
        let mut dirs: Vec<String> = vec![];
        let mut assets: Vec<String> = vec![];

        dirs.push(self.name.clone());

        loop {
            if let Some(dir) = dirs.pop() {
                if let Ok(entries) = read_dir(&("assets/".to_string() + &dir)) {
                    for entry in entries { if let Ok(entry) = entry {
                        let entry_name = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
        
                        if entry.path().is_dir() {
                            dirs.push(dir.clone() + "/" + &entry_name);
                        } else {
                            assets.push(dir.clone() + "/" + &entry_name);
                        }
        
                    }}
                }
            } else {
                break;
            }
        }

        for asset in assets.iter() {
            self.ids.insert(asset.clone(), self.assets.len());
            self.assets.push(asset_server.load(asset.clone() + "." + &self.extension));
        }
    }

    pub fn is_loading(
        &self,
        asset_server: &Res<AssetServer>,
    ) -> bool {
        let load_state = asset_server.get_group_load_state(self.assets.iter().map(|handle| handle.id()));

        if load_state == LoadState::Loading {
            true
        } else {
            false
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource)]
pub struct AssetLoader {
    // pub defs: Vec<Vec<u8>>,
    pub fonts: AssetType<Font>,
    pub images: AssetType<Image>,
    pub models: AssetType<Scene>,
    pub sounds: AssetType<bevy_kira_audio::AudioSource>,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            fonts: AssetType::<Font>::new("fonts", "ttf"),
            images: AssetType::<Image>::new("images", "png"),
            models: AssetType::<Scene>::new("models", "glb#Scene0"),
            sounds: AssetType::<bevy_kira_audio::AudioSource>::new("sounds", "ogg"),
        }
    }

    pub fn load_assets(
        &mut self,
        asset_server: &Res<AssetServer>,
    ) {
        self.fonts.load_assets(asset_server);
        self.images.load_assets(asset_server);
        self.models.load_assets(asset_server);
        self.sounds.load_assets(asset_server);
    }

    pub fn is_loading(
        &self,
        asset_server: &Res<AssetServer>,
    ) -> bool {
        if self.fonts.is_loading(asset_server) ||
           self.images.is_loading(asset_server) ||
           self.models.is_loading(asset_server) ||
           self.sounds.is_loading(asset_server) {
            true
        } else {
            false
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STARTUP SYSTEMS
fn stsys_init_asset_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut asset_loader = AssetLoader::new();
    asset_loader.load_assets(&asset_server);
    commands.insert_resource(asset_loader);
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_set_image_samplers(
    mut images: ResMut<Assets<Image>>,
) {
    for (_, mut image) in images.iter_mut() {
        image.sampler_descriptor = ImageSampler::Descriptor(
            SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Nearest,
                ..default()
            }
        )
    }
}