//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(InputState::default())
            .add_system(sys_update_input.in_base_set(CoreSet::PreUpdate));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// ENUMS
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
    MoveBack,
    MoveForward,

    FastMod,
    SlowMod,
    AltMod,
    CapsMod,
    MultiMod,

    PrimaryAction,
    SecondaryAction,
    TertiaryAction,

    Pause,
    
    Escape,
    Inventory,
    Settings,

    Z,

    Count,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputBindingKind {
    One,
    Any,
    All,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputStateFlag {
    JustReleased = 0x01,
    JustPressed  = 0x02,
    Pressed      = 0x04,
}
impl Flag for InputStateFlag {
    fn u8(self) -> u8 { self as u8 }
    fn u16(self) -> u16 { self as u16 }
    fn u32(self) -> u32 { self as u32 }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource)]
pub struct InputState {
    bindings: InputBindings,
    just_released: Bitmask,
    just_pressed: Bitmask,
    pressed: Bitmask,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            bindings: InputBindings::from_load_or_default(),
            just_released: Bitmask::from_cube_log2dim(3, false),
            just_pressed: Bitmask::from_cube_log2dim(3, false),
            pressed: Bitmask::from_cube_log2dim(3, false),
        }
    }
}

impl InputState {
    pub fn just_pressed(
        &self,
        input_action: InputAction,
    ) -> bool {
        self.just_pressed.is_bit_on(input_action as usize)
    }

    pub fn just_released(
        &self,
        input_action: InputAction,
    ) -> bool {
        self.just_released.is_bit_on(input_action as usize)
    }

    pub fn pressed(
        &self,
        input_action: InputAction,
    ) -> bool {
        self.pressed.is_bit_on(input_action as usize) ||
        self.just_pressed.is_bit_on(input_action as usize)
    }

    pub fn released(
        &self,
        input_action: InputAction,
    ) -> bool {
        self.pressed.is_bit_off(input_action as usize) &&
        self.just_pressed.is_bit_off(input_action as usize)
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// STRUCTS
#[derive(Serialize, Deserialize)]
pub struct InputBinding {
    kind: InputBindingKind,
    keys: Vec<KeyCode>,
    buttons: Vec<MouseButton>,
}

impl InputBinding {
    pub fn one() -> Self {
        Self { kind: InputBindingKind::One, keys: vec![], buttons: vec![] }
    }

    pub fn any() -> Self {
        Self { kind: InputBindingKind::Any, keys: vec![], buttons: vec![] }
    }

    pub fn all() -> Self {
        Self { kind: InputBindingKind::All, keys: vec![], buttons: vec![] }
    }

    pub fn with_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.keys = keys;
        self
    }

    pub fn with_buttons(mut self, buttons: Vec<MouseButton>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn state(&self, keyboard: &Res<Input<KeyCode>>, mouse: &Res<Input<MouseButton>>) -> u8 {
        let mut state: u8 = 0;
    
        match self.kind {
            InputBindingKind::One => {}
            InputBindingKind::Any => {
                for key in self.keys.iter() { state |= get_key_state(key, &keyboard); }
                for button in self.buttons.iter() { state |= get_button_state(button, &mouse); }
            }
            InputBindingKind::All => {
                for key in self.keys.iter() {
                    let input_state = get_key_state(key, &keyboard);
                    if input_state == 0 { return 0; }
                    state |= input_state;
                }
                for button in self.buttons.iter() {
                    let input_state = get_button_state(button, &mouse);
                    if input_state == 0 { return 0; }
                    state |= input_state;
                }

                if is_flag_on_u8(state, InputStateFlag::Pressed) {
                    if is_flag_on_u8(state, InputStateFlag::JustReleased) {
                        state = InputStateFlag::JustReleased as u8;
                    } else if is_flag_on_u8(state, InputStateFlag::JustPressed) {
                        state = InputStateFlag::JustPressed as u8 | InputStateFlag::Pressed as u8;
                    } else {
                        state = InputStateFlag::Pressed as u8;
                    }
                }
            }
        }
    
        state
    }
}

#[derive(Serialize, Deserialize)]
pub struct InputActionBinding(InputBinding, InputAction);

impl InputActionBinding {
    pub fn one(action: InputAction) -> Self {
        Self { 0: InputBinding { kind: InputBindingKind::One, keys: vec![], buttons: vec![] }, 1: action }
    }

    pub fn any(action: InputAction) -> Self {
        Self { 0: InputBinding { kind: InputBindingKind::Any, keys: vec![], buttons: vec![] }, 1: action }
    }

    pub fn all(action: InputAction) -> Self {
        Self { 0: InputBinding { kind: InputBindingKind::All, keys: vec![], buttons: vec![] }, 1: action }
    }

    pub fn with_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.0.keys = keys;
        self
    }

    pub fn with_buttons(mut self, buttons: Vec<MouseButton>) -> Self {
        self.0.buttons = buttons;
        self
    }

    pub fn state(&self, keyboard: &Res<Input<KeyCode>>, mouse: &Res<Input<MouseButton>>) -> u8 {
        self.0.state(keyboard, mouse)
    }
}

#[derive(Serialize, Deserialize)]
/// A combo binding cannot have more than 1 key that already exists in another single input!
/// 1, 2, 3, 4, 5 are bound
/// you can bind combos for shift + #, ctrl + #, etc. but you cannot bind # + #
pub struct InputBindings(Vec<InputActionBinding>);

impl Default for InputBindings {
    fn default() -> Self {
        Self {
            0: vec![
                InputActionBinding::any(InputAction::MoveLeft)   .with_keys(vec![KeyCode::A, KeyCode::Left]),
                InputActionBinding::any(InputAction::MoveRight)  .with_keys(vec![KeyCode::D, KeyCode::Right]),
                // InputActionBinding::any(InputAction::MoveDown)   .with_keys(vec![KeyCode::C, KeyCode::Left]),
                // InputActionBinding::any(InputAction::MoveUp)     .with_keys(vec![KeyCode::Space, KeyCode::Left]),
                InputActionBinding::any(InputAction::MoveBack)   .with_keys(vec![KeyCode::S, KeyCode::Down]),
                InputActionBinding::any(InputAction::MoveForward).with_keys(vec![KeyCode::W, KeyCode::Up]),

                InputActionBinding::any(InputAction::FastMod) .with_keys(vec![KeyCode::LShift]),
                InputActionBinding::any(InputAction::SlowMod) .with_keys(vec![KeyCode::LControl]),
                InputActionBinding::any(InputAction::AltMod)  .with_keys(vec![KeyCode::LAlt]),
                InputActionBinding::any(InputAction::CapsMod) .with_keys(vec![KeyCode::Capital]),
                InputActionBinding::any(InputAction::MultiMod).with_keys(vec![KeyCode::LShift]),
                
                InputActionBinding::any(InputAction::PrimaryAction)  .with_buttons(vec![MouseButton::Left]),
                InputActionBinding::any(InputAction::SecondaryAction).with_buttons(vec![MouseButton::Right]),
                InputActionBinding::any(InputAction::TertiaryAction) .with_buttons(vec![MouseButton::Middle]),

                InputActionBinding::any(InputAction::Pause).with_keys(vec![KeyCode::Space]),
                InputActionBinding::any(InputAction::Z).with_keys(vec![KeyCode::Z]),
                
                InputActionBinding::any(InputAction::Escape)   .with_keys(vec![KeyCode::Escape]),
                InputActionBinding::any(InputAction::Inventory).with_keys(vec![KeyCode::Tab]),
                InputActionBinding::any(InputAction::Settings) .with_keys(vec![KeyCode::Grave]),
            ],
        }
    }
}

impl InputBindings {
    pub fn from_load_or_default() -> Self {
        let mut input_bindings;

        if let Some(contents) = Data::try_read_file_to_string("data/controls.ron") {
            input_bindings = ron::from_str(&contents).unwrap();
        } else {
            input_bindings = InputBindings::default();
            if let Ok(contents) = Data::to_ron_string_pretty(&input_bindings) {
                Data::try_write_file("data/controls.ron", contents.as_bytes());
            }
        }

        input_bindings
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_update_input(
    mut input_state: ResMut<InputState>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut input_action_states: Vec<(u8, u16)> = vec![];
    for input_action_binding in input_state.bindings.0.iter() {
        let state = input_action_binding.state(&keyboard, &mouse);
        if state != 0 { input_action_states.push((state, input_action_binding.1 as u16)); }
    }
    
    input_state.just_released.set_off();
    input_state.just_pressed.set_off();
    input_state.pressed.set_off();

    for (state, action) in input_action_states {
        if is_flag_on_u8(state, InputStateFlag::JustReleased) { input_state.just_released.set_bit_on(action as usize); }
        if is_flag_on_u8(state, InputStateFlag::JustPressed)  { input_state.just_pressed.set_bit_on(action as usize); }
        if is_flag_on_u8(state, InputStateFlag::Pressed)      { input_state.pressed.set_bit_on(action as usize); }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
fn get_key_state(key: &KeyCode, keyboard: &Res<Input<KeyCode>>) -> u8 {
    if keyboard.just_released(*key) {
        InputStateFlag::JustReleased as u8
    } else if keyboard.just_pressed(*key) {
        InputStateFlag::JustPressed as u8 | InputStateFlag::Pressed as u8
    } else if keyboard.pressed(*key) {
        InputStateFlag::Pressed as u8
    } else {
        0
    }
}

fn get_button_state(button: &MouseButton, mouse: &Res<Input<MouseButton>>) -> u8 {
    if mouse.just_released(*button) {
        InputStateFlag::JustReleased as u8
    } else if mouse.just_pressed(*button) {
        InputStateFlag::JustPressed as u8 | InputStateFlag::Pressed as u8
    } else if mouse.pressed(*button) {
        InputStateFlag::Pressed as u8
    } else {
        0
    }
}