extern crate amethyst;
use amethyst::prelude::*;
use log::info;
use amethyst::{
    winit::VirtualKeyCode,
    input::{is_close_requested, is_key_down},
    ui::{UiFinder, UiEventType, UiText},
    ecs::prelude::Entity,
};

pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                // TODO: remove this
                let mut top_center_entity: Option<Entity> = None;

                data.world.exec(|ui_finder: UiFinder<'_>| {
                    top_center_entity = ui_finder.find("top-center");
                });

                if ui_event.event_type == UiEventType::Click && ui_event.target == top_center_entity.unwrap() {
                    let mut ui_text = data.world.write_storage::<UiText>();
                    let mut top_center_text = ui_text.get_mut(top_center_entity.unwrap()).unwrap();

                    top_center_text.text = String::from("start game");

                    return Trans::Pop;
                }
                Trans::None
            }
            StateEvent::Input(_input) => {
                // info!("Input Event detected: {:?}.", _input);
                Trans::None
            }
        }
    }
}
