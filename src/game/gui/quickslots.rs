//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct QuickslotsGuiPlugin;
impl Plugin for QuickslotsGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
                sys_update_quickslots_gui,
                sys_refresh_quickslots_gui,
            ));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
pub struct QuickslotsData {
    
}


//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
/// Add this on a camera to have the camera display the linked quickslots
pub struct QuickslotsGui(Entity);

impl QuickslotsGui {
    pub fn new(quickslots: Entity) -> Self {
        Self { 0: quickslots }
    }
}

#[derive(Component)]
pub struct QuickslotsGuiBar {
    position: Vec2,
    scale: f32,
    slots: u8,
}

#[derive(Component)]
pub struct QuickslotsGuiSlot;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// CHANGE SYSTEMS
fn sys_refresh_quickslots_gui(
    mut quickslot_events: EventWriter<QuickslotEvent>,
    mut quickslots_gui_query: Query<(&mut Style, &QuickslotsGui)>,
    quickslots_query: Query<&Quickslots, Changed<Quickslots>>,
) {
    for (mut style, gui) in quickslots_gui_query.iter_mut() {
        if let Ok(quickslots) = quickslots_query.get(gui.0) {

        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_quickslots_gui(
    mut quickslot_events: EventReader<QuickslotEvent>,
    mut quickslots_gui_query: Query<(&mut Style, &QuickslotsGui)>,
    quickslots_query: Query<&Quickslots>,
) {
    for (mut style, quickslots_gui) in quickslots_gui_query.iter_mut() {

    }
    
    // for quickslot_event in quickslot_events.iter() {
    //     let quickslots = if let Ok(quickslots) = quickslots_query.get(quickslot_event.0) { quickslots } else { continue; };
    //     quickslots.
    // }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
pub fn spawn_quickslots_gui(
    quickslots: Entity,
    commands: &mut Commands,
    asset_loader: &Res<AssetLoader>,
) {
    commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .insert(QuickslotsGui(quickslots))
        .with_children(|child_builder| {
            spawn_quickslots_bar(Vec2::new(5.0, 5.0), 1.5, 9, child_builder, asset_loader);
        });
}

fn spawn_quickslots_bar(
    position: Vec2,
    scale: f32,
    slots: u8,
    child_builder: &mut ChildBuilder,
    asset_loader: &Res<AssetLoader>,
) {
    child_builder.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                size: Size::new(
                    Val::Percent((3.0 * scale) * slots as f32),
                    Val::Percent(6.0 * scale),
                ),
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        })
        .insert(QuickslotsGuiBar { position, scale, slots })
        .with_children(|child_builder| {
            for slot in 0..slots {
                spawn_quickslot(slots, child_builder, asset_loader);
            }
        });
}

fn spawn_quickslot(
    slots: u8,
    child_builder: &mut ChildBuilder,
    asset_loader: &Res<AssetLoader>,
) {
    child_builder.spawn(GuiButtonBundle::new(ButtonBundle {
            style: Style {
                // position_type: PositionType::Relative,
                // align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                aspect_ratio: Some(1.0),
                size: Size::width(Val::Percent(100.0 / (slots + 1) as f32)),
                margin: UiRect::all(Val::Percent(0.5)),
                ..default()
            },
            image: asset_loader.images.get_handle("quickslot_00").into(),
            ..default()
        }))
        .insert(GuiButtonColors {
            clicked: Color::GREEN,
            hovered: Color::RED,
            base: Color::WHITE,
        })
        .insert(GuiButtonSounds {
            activated: Some(Sound2dEvent::new(asset_loader.sounds.get_handle("8bit/blip_high"), 1.0)),
            clicked: None,
            hovered: None,
        })
        .insert(QuickslotsGuiSlot)
        .with_children(|child_builder| {
            // ability icons, binding text
        });
}