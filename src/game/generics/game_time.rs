//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
use bevy::time::Stopwatch;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// PLUGIN
pub struct GameTimePlugin;
impl Plugin for GameTimePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(GameTime::new())
            .add_system(sys_listen_pause_input)
            .add_system(sys_update_game_time.in_schedule(CoreSchedule::FixedUpdate));
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// RESOURCES
#[derive(Resource)]
pub struct GameTime {
    locked: bool,
    paused: bool,
    previous: u32,
    steps_until_pause: u32,
    elapsed_steps: u32,
}

impl GameTime {
    fn new() -> Self {
        Self {
            locked: false,
            paused: false,
            previous: 0,
            steps_until_pause: 0,
            elapsed_steps: 0,
        }
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn steps_until_pause(&self) -> u32 {
        self.steps_until_pause
    }

    pub fn elapsed_steps(&self) -> u32 {
        self.elapsed_steps
    }

    pub fn delta_steps(&self) -> u32 {
        self.elapsed_steps - self.previous
    }

    pub fn toggle(&mut self) {
        if self.paused {
            self.unpause();
        } else {
            self.pause();
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn pause(&mut self) {
        if self.locked && self.steps_until_pause != 0 {
            return;
        } else {
            self.paused = true;
        }
    }

    pub fn unpause(&mut self) {
        if self.locked && self.steps_until_pause == 0 {
            return;
        } else {
            self.paused = false;
        }
    }

    pub fn set_steps_until_pause(&mut self, steps: u32) {
        self.steps_until_pause = steps;
    }

    fn tick(&mut self) {
        self.previous = self.elapsed_steps;
        if !self.paused { self.elapsed_steps += 1; }
        if self.locked {
            if self.steps_until_pause == 0 {
                self.paused = true;
            } else {
                self.steps_until_pause -= 1;
            }
        }
    }
}

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// SYSTEMS
fn sys_listen_pause_input(
    mut game_time: ResMut<GameTime>,
    input_state: Res<InputState>,
) {
    if input_state.just_pressed(InputAction::Pause) {
        game_time.toggle();
    }
}

fn sys_update_game_time(
    mut game_time: ResMut<GameTime>,
    input_state: Res<InputState>,
    fixed_time: Res<FixedTime>,
) {
    game_time.tick();
}