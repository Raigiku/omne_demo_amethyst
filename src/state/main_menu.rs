use amethyst::{
    audio::output::init_output,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiTransform},
    winit::VirtualKeyCode,
};

use crate::resource;
use crate::state;

pub struct MainMenu;

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        init_output(&mut world.res);
        world.add_resource::<resource::Game>(resource::Game::new());
        world.exec(|mut creator: UiCreator<'_>| creator.create("ui/main_menu.ron", ()));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
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

                if self.pressed_play_button(world, selected_component.unwrap(), ui_event) {
                    return Trans::Push(Box::new(state::Gameplay::new()));
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
}

impl MainMenu {
    fn pressed_play_button(
        &self,
        world: &World,
        selected_component: &UiTransform,
        ui_event: &UiEvent,
    ) -> bool {
        if selected_component.id == "play_button" && ui_event.event_type == UiEventType::Click {
            world.write_resource::<resource::Game>().current_state = resource::GameState::Gameplay;
            return true;
        } else {
            return false;
        }
    }
}
