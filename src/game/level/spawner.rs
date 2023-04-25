//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::pbr::NotShadowCaster;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<SpawnEvent>()
            .add_system(evsys_receive_spawn_events);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub enum SpawnEvent {
    Despawn(Entity),
    UnitPropertiesDisplay(Entity),
    Actor(IVec3, u8),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENT SYSTEMS
fn evsys_receive_spawn_events(
    mut spawn_events: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    entity_tree_query: Query<&EntityTree, With<LevelTree>>,
    defs: Res<Defs>,
) {
    let mut entities = entity_tree_query.single().get_accessor();
    for spawn_event in spawn_events.iter() {
        match spawn_event {
            SpawnEvent::Despawn(entity) => { commands.entity(*entity).despawn_recursive(); }
            SpawnEvent::UnitPropertiesDisplay(entity) => { spawn_unit_properties_display(entity, &mut commands); }
            SpawnEvent::Actor(coord, id) => { try_spawn_unit(coord, *id, &mut commands, &mut entities, &mut meshes, &mut materials, &defs); }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
fn spawn_unit_properties_display(
    unit: &Entity,
    commands: &mut Commands,
) {
    println!("Spawning Unit Properties Display");
    // GuiTextBundle::new("", "caveat/bold", 24.0, &Color::WHITE, asset_loader)
    let display = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(WorldspaceFollow::new(*unit, Vec2::new(0.0, 64.0)))
        .insert(WorldspaceScale::new(Val::Percent(7.5), Val::Percent(5.0)))
        .insert(SelectedVisual(*unit))
        .with_children(|child_builder| {
            spawn_unit_property_bar(unit, UnitProperty::Health, child_builder);
            spawn_unit_property_bar(unit, UnitProperty::Stamina, child_builder);
            spawn_unit_property_bar(unit, UnitProperty::Mana, child_builder);
        })
        .id();
}

fn spawn_unit_property_bar(
    unit_entity: &Entity,
    property: UnitProperty,
    child_builder: &mut ChildBuilder,
) {
    // GuiTextBundle::new("", "caveat/bold", 24.0, &Color::WHITE, asset_loader)
    child_builder.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                // align_items: AlignItems::Start,
                // justify_content: JustifyContent::Start,
                size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                margin: UiRect {
                    top: Val::Percent(1.5),
                    bottom: Val::Percent(1.5),
                    ..default()
                },
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|child_builder| {
            child_builder.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: match property {
                        UnitProperty::Health => { Color::RED }
                        UnitProperty::Stamina => { Color::SEA_GREEN }
                        UnitProperty::Mana => { Color::MIDNIGHT_BLUE }
                    }.into(),
                    ..default()
                })
                .insert(BarBinding::UnitProperty(*unit_entity, property));
        });
}

fn spawn_unit_selection_marker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    defs: &Res<Defs>,
) -> Entity {
    commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::splat(CUBE_HALF_DIM * 2.0), flip: false })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba_u8(255, 200, 200, 100) * 5.0,
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Multiply,
                unlit: true,
                ..default()
            }),
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(Vec3::NEG_Y * 0.49).looking_to(Vec3::NEG_Y, Vec3::X),
            ..default()
        })
        .insert(NotShadowCaster)
        .insert(SelectionMarker)
        .insert(Name::new("Selection Marker"))
        .id()
}

fn try_spawn_unit(
    coord: &IVec3,
    id: u8,
    commands: &mut Commands,
    entities: &mut Accessor<Option<Entity>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    defs: &Res<Defs>,
) -> Option<Entity> {
    let mut rng = thread_rng();
    if entities.get_value(coord).is_none() {
        let selection_marker = spawn_unit_selection_marker(commands, meshes, materials, defs);
        let entity = Some(commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0, })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::RED,
                        perceptual_roughness: 1.0,
                        alpha_mode: AlphaMode::Multiply,
                        unlit: true,
                        ..default()
                    }),
                    ..default()
                })
            .insert(UnitBundle::from_coord(coord))
            .insert(UnitSelectable::new(selection_marker))
            .insert(UnitOrderable::default())
            .insert(UnitMover::default())
            .insert(UnitActioner::default())
            .insert(BaseUnitPropertiesBundle {
                health: Health::new(rng.gen_range(0..=10), 10),
                stamina: Stamina::new(rng.gen_range(0..=10), 10),
                mana: Mana::new(rng.gen_range(0..=10), 10),
            })
            .insert(PlayerControlled)
            .insert(Name::new("Actor"))
            .push_children(&[selection_marker])
            .id());
        entities.set_value_on(coord, &entity);
        entity
    } else {
        None
    }
}