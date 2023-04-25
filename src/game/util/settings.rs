//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Settings::from_load_or_default());
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource, Serialize, Deserialize)]
pub struct Settings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub bgm_volume: f32,

    pub camera_speed: f32,
    pub camera_pivot_speed: f32,
    pub camera_zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub camera_fast_mult: f32,
    pub camera_slow_mult: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 1.0,
            bgm_volume: 1.0,

            camera_speed: 5.0,
            camera_pivot_speed: 1.0,
            camera_zoom_speed: 0.5,
            min_zoom: 1.0,
            max_zoom: 100.0,
            camera_fast_mult: 2.0,
            camera_slow_mult: 0.5,
        }
    }
}

impl Settings {
    pub fn from_load_or_default() -> Self {
        let mut settings: Self;

        if let Some(contents) = Data::try_read_file_to_string("data/settings.ron") {
            settings = ron::from_str(&contents).unwrap();
        } else {
            settings = Settings::default();
            if let Ok(contents) = Data::to_ron_string_pretty(&settings) {
                Data::try_write_file("data/settings.ron", contents.as_bytes());
            }
        }

        settings
    }
}