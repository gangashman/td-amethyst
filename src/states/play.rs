extern crate amethyst;
use amethyst::prelude::*;
use amethyst::{
    winit::VirtualKeyCode,
    input::{is_close_requested, is_key_down},
    core::Time,
};
use crate::unit::spawn_unit;
use crate::map::{LevelInfo};
// use log::info;

pub struct PlayState {
    pub wave: u32,
    pub index_spawn: u32,
    pub last_spawn_time: u128,
}

impl PlayState {
    pub fn new(wave: u32) -> PlayState {
        PlayState{
            wave: wave,
            index_spawn: 0,
            last_spawn_time: 0,
        }
    }
}

impl SimpleState for PlayState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        return Trans::None;
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let current_time = data.world.fetch::<Time>().absolute_time().as_millis();

        // If pass enough time from last spawn
        let spawn_speed = data.world.fetch::<LevelInfo>().spawn_speed;
        if self.last_spawn_time + spawn_speed as u128 <= current_time {

            // If any units left to spawn
            if self.index_spawn < data.world.fetch::<LevelInfo>().get_units_count(self.wave) {

                spawn_unit(data.world, 0, self.index_spawn);
                self.index_spawn += 1;
                self.last_spawn_time = current_time;
            }
        }
        Trans::None
    }
}
