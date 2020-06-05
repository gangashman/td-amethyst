extern crate amethyst;
use amethyst::prelude::*;
use amethyst::{
    winit::VirtualKeyCode,
    input::{is_close_requested, is_key_down},
};
use crate::unit::spawn_wave_units;
// use log::info;

pub struct PlayState {
    pub game_speed: f32,
    pub wave: u32,
}

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        spawn_wave_units(data.world, 0);
    }

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

        // let state_change = match &event {
        //     StateEvent::Ui(ui_event) => {
        //         let entity = data.world.exec(|ui_finder: UiFinder<'_>| { ui_finder.find("top-center") });
        //         ui_event.event_type == UiEventType::Click && ui_event.target.id() == entity.unwrap().id()
        //     },
        //     StateEvent::Input(input) => {
        //         match input {
        //             InputEvent::ButtonPressed(dir) => dir == &Button::Key(VirtualKeyCode::Space),
        //             _ => false
        //         }
        //     },
        //     _ => false
        // };
        // if state_change {
        //     let entity = data.world.exec(|ui_finder: UiFinder<'_>| { ui_finder.find("top-center") });
        //     let mut ui_text = data.world.write_storage::<UiText>();
        //     let mut top_center_text = ui_text.get_mut(entity.unwrap()).unwrap();
        //     top_center_text.text = String::from("start game");

        //     return Trans::Pop;
        // }
        return Trans::None;
    }
}
