//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::ui::RelativeCursorPosition;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct ButtonGuiPlugin;
impl Plugin for ButtonGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonEvent>()
            .add_systems((
                sys_update_button_state,
                sys_update_button_functions.after(sys_update_button_state),
                sys_update_button_sounds.after(sys_update_button_state),
                sys_update_button_colors.after(sys_update_button_state),
            ).after(PlayerSet::Last));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// BUNDLES
#[derive(Bundle)]
pub struct GuiButtonBundle {
    pub button_bundle: ButtonBundle,
    relative_cursor_position: RelativeCursorPosition,
    flags: GuiButtonFlags,
}

impl GuiButtonBundle {
    pub fn new(button_bundle: ButtonBundle) -> Self {
        Self {
            button_bundle,
            relative_cursor_position: RelativeCursorPosition::default(),
            flags: GuiButtonFlags::default(),
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUM
pub enum ButtonFlag {
    /// Cursor is inside the button
    Cursor = 0x01,
    /// The user clicked and moved their cursor off the button, but did not release yet
    Primed = 0x02,
    /// The user used tab or arrow keys to highlight the button. Wasn't sure what to call this. So it's KEYED
    Keyed  = 0x04,
}
impl Flag for ButtonFlag { fn u8(self) -> u8 { self as u8 } fn u16(self) -> u16 { self as u16 } fn u32(self) -> u32 { self as u32 } }

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// COMPONENTS
#[derive(Component)]
pub struct GuiButtonColors {
    pub clicked: Color,
    pub hovered: Color,
    pub base: Color,
}

impl Default for GuiButtonColors {
    fn default() -> Self {
        Self {
            clicked: Color::rgb(0.35, 0.75, 0.35),
            hovered: Color::rgb(0.20, 0.20, 0.20),
            base: Color::rgb(0.15, 0.15, 0.15),
        }
    }
}

#[derive(Component)]
pub struct GuiButtonSounds {
    pub activated: Option<Sound2dEvent>,
    pub clicked: Option<Sound2dEvent>,
    pub hovered: Option<Sound2dEvent>,
}

#[derive(Default, Component)]
pub struct GuiButtonFlags(u8);

#[derive(Component)]
pub enum ButtonFunction {
    NextState(AppState),
    Quickslot(u8),
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// EVENTS
pub struct ButtonEvent(Entity);

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_button_state(
    mut button_events: EventWriter<ButtonEvent>,
    mut button_query: Query<(Entity, &mut GuiButtonFlags, &RelativeCursorPosition, &Interaction)>,
    input_state: Res<InputState>,
) {
    for (entity, mut flags, cursor, interaction) in button_query.iter_mut() {
        if let Some(position) = cursor.normalized {
            if position.x >= 0.0 && position.x <= 1.0 && position.y >= 0.0 && position.y <= 1.0 {
                flags.0 |= ButtonFlag::Cursor.u8();
                if is_flag_on_u8(flags.0, ButtonFlag::Primed) && input_state.just_released(InputAction::PrimaryAction) {
                    button_events.send(ButtonEvent(entity));
                }
            } else { flags.0 &= !ButtonFlag::Cursor.u8(); }
        } else { flags.0 &= !ButtonFlag::Cursor.u8(); }

        match *interaction {
            Interaction::Clicked => { flags.0 |= ButtonFlag::Primed.u8(); }
            _ => { if input_state.released(InputAction::PrimaryAction) { flags.0 &= !ButtonFlag::Primed.u8(); } }
        }
    }
}

fn sys_update_button_colors(
    mut button_query: Query<(&mut BackgroundColor, &GuiButtonColors, &GuiButtonFlags, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (mut bg_color, button_colors, flags, interaction) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if is_flag_on_u8(flags.0, ButtonFlag::Cursor) {
                    *bg_color = button_colors.clicked.into();
                } else {
                    *bg_color = button_colors.base.into();
                }
            }
            Interaction::Hovered => {
                *bg_color = button_colors.hovered.into();
            }
            Interaction::None => {
                *bg_color = button_colors.base.into();
            }
        }
    }
}

fn sys_update_button_sounds(
    mut sound_2d_events: EventWriter<Sound2dEvent>,
    mut button_events: EventReader<ButtonEvent>,
    mut button_query: Query<(&GuiButtonSounds, &GuiButtonFlags, &Interaction)>,
) {
    for (button_sounds, flags, interaction) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if is_flag_on_u8(flags.0, ButtonFlag::Cursor) && is_flag_off_u8(flags.0, ButtonFlag::Primed) {
                    if let Some(sound) = &button_sounds.clicked { sound_2d_events.send(sound.clone()); }
                }
            }
            Interaction::Hovered => {
                if let Some(sound) = &button_sounds.hovered { sound_2d_events.send(sound.clone()); }
            }
            Interaction::None => {}
        }
    }

    for button_event in button_events.iter() {
        if let Ok((button_sounds, _, _)) = button_query.get(button_event.0) {
            if let Some(sound) = &button_sounds.activated { sound_2d_events.send(sound.clone()); }
        }
    }
}

fn sys_update_button_functions(
    mut button_events: EventReader<ButtonEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    button_query: Query<&ButtonFunction>,
) {
    for button_event in button_events.iter() {
        let function = if let Ok(function) = button_query.get(button_event.0) { function } else { continue; };
        match function {
            ButtonFunction::NextState(state) => { next_state.set(*state); }
            ButtonFunction::Quickslot(index) => {  }
        }
    }
}