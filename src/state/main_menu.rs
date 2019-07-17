use amethyst::{
    audio::output::init_output,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiTransform, UiEventType},
    winit::VirtualKeyCode,
};

use crate::state::GameplayState;

pub struct MainMenuState;

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        init_output(&mut world.res);
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", ())
        });
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                let ui_transform_components = world.read_storage::<UiTransform>();
                let selected_component = ui_transform_components.get(ui_event.target);
                if selected_component.unwrap().id == "play_button" && ui_event.event_type == UiEventType::Click {
                    return Trans::Push(Box::new(GameplayState {progress_counter: Some(Default::default())}));
                }
                Trans::None
            }
            _ => Trans::None
        }
    }
}