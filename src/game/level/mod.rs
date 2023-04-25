//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

mod daytime;
pub use daytime::*;
mod editor;
pub use editor::*;
mod generator;
pub use generator::*;
mod spawner;
pub use spawner::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DaytimePlugin)
            .add_plugin(EditorPlugin)
            .add_plugin(GeneratorPlugin)
            .add_plugin(SpawnerPlugin)
            .add_startup_system(stsys_spawn_level_tree)
            .add_system(onsys_init_level.in_schedule(OnEnter(AppState::MainMenu)));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STARTUP SYSTEMS
fn stsys_spawn_level_tree(
    mut commands: Commands,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_loader: Res<AssetLoader>,
    random: Res<Random>,
) {
    commands.spawn(LevelTreeBundle::new(Voxel::default(), VoxelMaterials::new_world_materials(&mut materials, &asset_loader)));
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SCHEDULE SYSTEMS
fn onsys_init_level(
    mut spawn_events: EventWriter<SpawnEvent>,
    mut level_tree_query: Query<&mut VoxelTree, With<LevelTree>>,
    asset_loader: Res<AssetLoader>,
    random: Res<Random>,
) {
    let mut voxels = level_tree_query.single().get_accessor();
    
    let mut rng = random.get_rng();
    let dirt = Voxel::from_matter_id(1);
    let grass = Voxel::from_matter_id(2);
    let tilled_dirt = Voxel::from_matter_id(3);
    let stone = Voxel::from_matter_id(4);

    let horizontal_dim = 50;
    for z in -horizontal_dim..=horizontal_dim { for y in -5..=5 { for x in -horizontal_dim..=horizontal_dim {
        let chance = rng.gen_range(0..4);
        match chance {
            0 => { voxels.set_value_on(&IVec3::new(x, y, z), &dirt); },
            1 => { voxels.set_value_on(&IVec3::new(x, y, z), &grass); },
            2 => { voxels.set_value_on(&IVec3::new(x, y, z), &tilled_dirt); },
            3 => { voxels.set_value_on(&IVec3::new(x, y, z), &stone); },
            _ => { voxels.set_value_on(&IVec3::new(x, y, z), &grass); },
        }
    }}}

    // Unit Test
    let mut rng = thread_rng();
    for z in -4..=4 { for x in -4..=4 {
        if rng.gen::<f32>() > 0.9 { spawn_events.send(SpawnEvent::Actor(IVec3::new(x, 6, z), 0)); }
    }}
    
}