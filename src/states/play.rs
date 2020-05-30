extern crate amethyst;
use amethyst::prelude::*;

struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
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
            StateEvent::Ui(_ui_event) => {
                info!("[PLAY] You just interacted with a ui element: {:?}", _ui_event);
                Trans::None
            }
            StateEvent::Input(_input) => {
                // info!("Input Event detected: {:?}.", _input);
                Trans::None
            }
        }
    }
}
