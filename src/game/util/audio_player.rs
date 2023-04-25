//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy_kira_audio::prelude::*;
use std::char::MAX;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CONSTANTS
const MAX_ACTIVE_2D_SOUNDS: usize = 64;
const MAX_ACTIVE_3D_SOUNDS: usize = 32;
const MAX_ACTIVE_BGM: usize = 16;

type KiraAudio = bevy_kira_audio::Audio;
type Sound = bevy_kira_audio::AudioSource;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct AudioPlayerPlugin;
impl Plugin for AudioPlayerPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(SpacialAudio { max_distance: 25.0 })
            .insert_resource(ActiveBGM::default())
            .add_plugin(AudioPlugin)
            .add_event::<Sound2dEvent>()
            .add_event::<Sound3dEvent>()
            .add_event::<BGMEvent>()
            .add_systems((
                    evsys_receive_sound_2d,
                    evsys_receive_sound_3d,
                    evsys_receive_bgm,
                ));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource)]
struct ActiveBGM(Vec<BGMInstance>);

impl Default for ActiveBGM {
    fn default() -> Self {
        Self { 0: vec![] }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct Sound3dListener;
#[derive(Component)]
struct Sound3dEmitter(Handle<AudioInstance>);

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
struct BGMInstance {
    instance_handle: Handle<AudioInstance>,
    layer: u8,
}

impl BGMInstance {
    fn new(instance_handle: Handle<AudioInstance>, layer: u8) -> Self {
        Self { instance_handle, layer }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
//================================-================================-================================ 
// Sound2dEvent
#[derive(Clone)]
pub struct Sound2dEvent {
    sound: Handle<Sound>,
    volume: f32,
}

impl Sound2dEvent {
    pub fn new(
        sound: Handle<Sound>,
        volume: f32,
    ) -> Self {
        Self {
            sound,
            volume,
        }
    }
}

//================================-================================-================================ 
// Sound3dEvent
#[derive(Clone)]
pub struct Sound3dEvent {
    sound: Handle<Sound>,
    volume: f32,
    position: Vec3,
}

impl Sound3dEvent {
    pub fn new(
        sound: Handle<Sound>,
        position: Vec3,
        volume: f32,
    ) -> Self {
        Self {
            sound,
            volume,
            position,
        }
    }
}

//================================-================================-================================ 
// BGMEvent
#[derive(Clone)]
pub struct BGMEvent {
    sound: Handle<Sound>,
    volume: f32,
    layer: u8,
}

impl BGMEvent {
    pub fn new(
        sound: Handle<Sound>,
        volume: f32,
        layer: u8,
    ) -> Self {
        Self {
            sound,
            volume,
            layer,
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT READER SYSTEMS
//================================-================================-================================ 
// Sound2dEvent
fn evsys_receive_sound_2d(
    mut sound_2d_events: EventReader<Sound2dEvent>,
    audio: Res<KiraAudio>,
    settings: Res<Settings>,
) {
    let base_volume = settings.master_volume * settings.sfx_volume;
    for sound_2d in sound_2d_events.iter() {
        audio.play(sound_2d.sound.clone())
            .with_volume((base_volume * sound_2d.volume) as f64);
    }
}

//================================-================================-================================ 
// Sound3dEvent
fn evsys_receive_sound_3d(
    mut commands: Commands,
    mut sound_3d_events: EventReader<Sound3dEvent>,
    audio: Res<KiraAudio>,
    settings: Res<Settings>,
) {
    let base_volume = settings.master_volume * settings.sfx_volume;
    for sound_3d in sound_3d_events.iter() {
        let instance_handle = audio.play(sound_3d.sound.clone())
            .with_volume((base_volume * sound_3d.volume) as f64)
            .handle();

        commands.spawn(TransformBundle {
                local: Transform::from_translation(sound_3d.position),
                ..default()
            })
            .insert(AudioEmitter {
                instances: vec![instance_handle],
            });
    }
}

//================================-================================-================================ 
// BGMEvent
fn evsys_receive_bgm(
    mut bgm_events: EventReader<BGMEvent>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut active_bgm: ResMut<ActiveBGM>,
    audio: Res<KiraAudio>,
    settings: Res<Settings>,
) {
    let base_volume = settings.master_volume * settings.bgm_volume;
    
    for bgm in bgm_events.iter() {
        let mut remove_index = None;
        for (i, bgm_instance) in active_bgm.0.iter().enumerate() {
            if bgm_instance.layer == bgm.layer {
                if let Some(instance) = audio_instances.get_mut(&bgm_instance.instance_handle) {
                    instance.stop(AudioTween::default());
                }
                remove_index = Some(i);
                break;
            }
        }

        if let Some(remove_index) = remove_index {
            active_bgm.0.swap_remove(remove_index);
        }
        
        let instance_handle = audio.play(bgm.sound.clone())
            .with_volume((base_volume * bgm.volume) as f64)
            .looped()
            .handle();

        active_bgm.0.push(BGMInstance::new(instance_handle, bgm.layer));
    }
}