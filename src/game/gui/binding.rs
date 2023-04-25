//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct BindingGuiPlugin;
impl Plugin for BindingGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sys_update_text_bindings)
            .add_system(sys_update_bar_bindings);
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub enum BarBinding {
    UnitProperty(Entity, UnitProperty),
}

#[derive(Component)]
pub enum TextBinding {
    GameTime(GameTimeValue),
}

impl TextBinding {
    fn update_text(text: &mut Text, string: String) {
        text.sections[0].value = string;
    }
}

pub enum GameTimeValue {
    TimeToPause,
    Elapsed,
}

impl GameTimeValue {
    fn to_string(
        &self,
        game_time: &Res<GameTime>,
    ) -> String {
        match self {
            GameTimeValue::TimeToPause => {
                game_time.steps_until_pause().to_string()
            }
            GameTimeValue::Elapsed => {
                game_time.elapsed_steps().to_string()
            }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_text_bindings(
    mut text_query: Query<(&mut Text, &TextBinding)>,
    game_time: Res<GameTime>,
) {
    for (mut text, bind_text) in text_query.iter_mut() {
        match bind_text {
            TextBinding::GameTime(value) => {TextBinding::update_text(&mut text, value.to_string(&game_time))}
        }
    }
}

fn sys_update_bar_bindings(
    mut bar_query: Query<(&BarBinding, &mut Style)>,
    health_query: Query<(&Health)>,
    stamina_query: Query<(&Stamina)>,
    mana_query: Query<(&Mana)>,
) {
    for (binding, mut style) in bar_query.iter_mut() {
        match binding {
            BarBinding::UnitProperty(unit_entity, property) => {
                let percent = unit_property_percent(unit_entity, *property, &(&health_query, &stamina_query, &mana_query));
                style.size.width = Val::Percent(percent * 100.0);
            }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
fn unit_property_percent(
    unit_entity: &Entity,
    property: UnitProperty,
    property_queries: &(&Query<(&Health)>, &Query<(&Stamina)>, &Query<(&Mana)>),
) -> f32 {
    match property {
        UnitProperty::Health => {
            if let Ok(health) = property_queries.0.get(*unit_entity) { return health.percent(); }
        }
        UnitProperty::Stamina => {
            if let Ok(stamina) = property_queries.1.get(*unit_entity) { return stamina.percent(); }
        }
        UnitProperty::Mana => {
            if let Ok(mana) = property_queries.2.get(*unit_entity) { return mana.percent(); }
        }
    }

    0.0
}